#![allow(clippy::upper_case_acronyms)]

use std::fs::File;
use std::io;
use std::os::windows::io::FromRawHandle;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::JoinHandle;

use super::{
    client::PipeClient, CreateEventW, CreateNamedPipeW, Overlapped, FILE_FLAG_OVERLAPPED, HANDLE,
    INVALID_HANDLE_VALUE, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE,
    PIPE_UNLIMITED_INSTANCES,
};
use super::{
    CloseHandle, ConnectNamedPipe, GetLastError, GetOverlappedResult, ERROR_IO_PENDING,
    ERROR_PIPE_CONNECTED, PIPE_WAIT,
};
use crate::ipc::pipe::{PipeClientImpl, PipeServerImpl};
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::session::SessionManager;
use crate::{client_event, local_event};

pub struct PipeServer {
    session_manager: Arc<SessionManager>,
    pipe_name: String,
    tx: Sender<Message>,
    next_client_id: Arc<AtomicU32>,
    running: Arc<AtomicBool>,
    thread_handle: Option<JoinHandle<()>>,
}

impl PipeServerImpl for PipeServer {
    fn new(pipe_name: &str, tx: Sender<Message>, session_manager: Arc<SessionManager>) -> Self {
        Self {
            session_manager,
            pipe_name: pipe_name.to_string(),
            tx,
            next_client_id: Arc::new(AtomicU32::new(0)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    fn start(&mut self) -> io::Result<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let pipe_name = self.pipe_name.clone();
        let session_manager = Arc::clone(&self.session_manager);
        let next_client_id = Arc::clone(&self.next_client_id);
        let running = Arc::clone(&self.running);
        let tx = self.tx.clone();
        let notified = Arc::new(AtomicBool::new(false));

        self.thread_handle = Some(std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                if let Ok(handle) = PipeServer::create_pipe_instance(&pipe_name) {
                    if !notified.load(Ordering::SeqCst) {
                        println!("Ready");
                        notified.store(true, Ordering::SeqCst);
                    }

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

                        let connect_result = ConnectNamedPipe(handle, &mut overlapped);
                        if connect_result == 0 {
                            let error = GetLastError();
                            if error != ERROR_IO_PENDING && error != ERROR_PIPE_CONNECTED {
                                CloseHandle(handle);
                                CloseHandle(h_event);
                                tx.send(local_event!(
                                    0,
                                    Error,
                                    ErrorEvent::new(Box::new(io::Error::from_raw_os_error(
                                        error as _,
                                    )))
                                ))
                                .ok();
                                continue;
                            }
                        }

                        let mut bytes_transferred = 0;
                        if GetOverlappedResult(handle, &mut overlapped, &mut bytes_transferred, 1)
                            == 0
                        {
                            let error = GetLastError();
                            CloseHandle(handle);
                            CloseHandle(h_event);
                            tx.send(local_event!(
                                0,
                                Error,
                                ErrorEvent::new(Box::new(io::Error::from_raw_os_error(error as _)))
                            ))
                            .ok();
                            continue;
                        }

                        let client_id = next_client_id.fetch_add(1, Ordering::SeqCst);
                        let mut client = PipeClient::new(
                            client_id,
                            File::from_raw_handle(handle as _),
                            tx.clone(),
                        );
                        client.start_read_thread().ok();
                        session_manager.create_session(client_id, client);
                        tx.send(client_event!(client_id, Connect)).ok();

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
    }

    fn broadcast(&self, data: &[u8]) -> io::Result<()> {
        let mut sessions = self.session_manager.sessions.write().unwrap();
        for session in sessions.values_mut() {
            if let Some(client) = session.get_pipe_client_mut() {
                client.write(data)?;
            }
        }
        Ok(())
    }

    fn write_to(&self, client_id: u32, data: &[u8]) -> io::Result<()> {
        let mut sessions = self.session_manager.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(&client_id) {
            if let Some(client) = session.get_pipe_client_mut() {
                return client.write(data);
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Client not found"))
    }

    fn disconnect(&self, client_id: u32) -> io::Result<()> {
        self.session_manager.remove_session(client_id);
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
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
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
