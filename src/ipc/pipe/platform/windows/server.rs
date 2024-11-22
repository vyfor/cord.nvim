#![allow(clippy::upper_case_acronyms)]

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::os::windows::io::FromRawHandle;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use super::{
    client::PipeClient, CreateEventW, CreateNamedPipeW, Overlapped, FILE_FLAG_OVERLAPPED, HANDLE,
    INVALID_HANDLE_VALUE, LPVOID, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES, WAIT_OBJECT_0,
};
use super::{
    CloseHandle, ConnectNamedPipe, GetLastError, WaitForSingleObject, ERROR_IO_PENDING,
    ERROR_PIPE_CONNECTED, INFINITE,
};
use crate::ipc::pipe::{PipeClientImpl, PipeServerImpl};
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::{client_event, local_event};

pub struct PipeServer {
    pipe_name: String,
    tx: Sender<Message>,
    clients: Arc<Mutex<HashMap<u32, PipeClient>>>,
    next_client_id: Arc<AtomicU32>,
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl PipeServerImpl for PipeServer {
    fn new(pipe_name: &str, tx: Sender<Message>) -> Self {
        Self {
            pipe_name: pipe_name.to_string(),
            tx,
            clients: Arc::new(Mutex::new(HashMap::new())),
            next_client_id: Arc::new(AtomicU32::new(1)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    fn start(&mut self) -> io::Result<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let pipe_name = self.pipe_name.clone();
        let clients = Arc::clone(&self.clients);
        let next_client_id = Arc::clone(&self.next_client_id);
        let running = Arc::clone(&self.running);
        let tx = self.tx.clone();

        self.thread_handle = Some(std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                if let Ok(handle) = PipeServer::create_pipe_instance(&pipe_name) {
                    unsafe {
                        let h_event =
                            CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
                        if h_event.is_null() {
                            CloseHandle(handle);
                            tx.send(local_event!(
                                0,
                                Error,
                                ErrorEvent::new(Box::new(io::Error::last_os_error()))
                            ))
                            .ok();
                            continue;
                        }

                        let mut overlapped = Overlapped {
                            internal: 0,
                            internal_high: 0,
                            offset: 0,
                            offset_high: 0,
                            h_event,
                        };

                        let connect_result =
                            ConnectNamedPipe(handle, &mut overlapped as *mut _ as LPVOID);
                        if connect_result == 0 {
                            let error = GetLastError();
                            if error != ERROR_IO_PENDING && error != ERROR_PIPE_CONNECTED {
                                CloseHandle(handle);
                                CloseHandle(h_event);
                                tx.send(local_event!(
                                    0,
                                    Error,
                                    ErrorEvent::new(Box::new(io::Error::from_raw_os_error(
                                        error as _
                                    )))
                                ))
                                .ok();
                                continue;
                            }
                        }

                        if WaitForSingleObject(overlapped.h_event, INFINITE) != WAIT_OBJECT_0 {
                            CloseHandle(handle);
                            CloseHandle(h_event);
                            continue;
                        }

                        let client_id = next_client_id.fetch_add(1, Ordering::SeqCst);
                        let mut client = PipeClient::new(
                            client_id,
                            File::from_raw_handle(handle as _),
                            tx.clone(),
                        );
                        client.start_read_thread().ok();
                        tx.send(client_event!(client_id, Connect)).ok();
                        clients.lock().unwrap().insert(client_id, client);

                        CloseHandle(h_event);
                    }
                }
            }
        }));

        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take() {
            drop(handle);
        }
        self.clients.lock().unwrap().clear();
    }

    fn broadcast(&self, data: &[u8]) -> io::Result<()> {
        let mut clients = self.clients.lock().unwrap();
        let mut failed_clients = Vec::new();

        for (client_id, client) in clients.iter_mut() {
            if client.write(data).is_err() {
                failed_clients.push(*client_id);
            }
        }

        for client_id in failed_clients {
            clients.remove(&client_id);
        }

        Ok(())
    }

    fn write_to(&self, client_id: u32, data: &[u8]) -> io::Result<()> {
        self.clients
            .lock()
            .unwrap()
            .get_mut(&client_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Client not found"))?
            .write(data)
    }

    fn disconnect(&self, client_id: u32) -> io::Result<()> {
        self.clients.lock().unwrap().remove(&client_id);
        Ok(())
    }
}

impl PipeServer {
    fn create_pipe_instance(pipe_name: &str) -> io::Result<HANDLE> {
        let wide_name: Vec<u16> = format!("\\\\.\\pipe\\{}", pipe_name)
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let handle = unsafe {
            CreateNamedPipeW(
                wide_name.as_ptr(),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE,
                PIPE_UNLIMITED_INSTANCES,
                1024 * 16,
                1024 * 16,
                0,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(handle)
        }
    }
}

impl Drop for PipeServer {
    fn drop(&mut self) {
        self.stop();
    }
}
