use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::os::windows::io::FromRawHandle;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::ipc::pipe::{PipeClientImpl, PipeServerImpl};
use crate::messages::message::{Event, LocalMessage, Message};

use super::client::PipeClient;

type HANDLE = *mut std::ffi::c_void;
type DWORD = u32;
type BOOL = i32;
type LPCWSTR = *const u16;
type LPVOID = *mut std::ffi::c_void;

const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const ERROR_PIPE_CONNECTED: DWORD = 535;
const PIPE_ACCESS_DUPLEX: DWORD = 0x00000003;
const PIPE_TYPE_MESSAGE: DWORD = 0x00000004;
const PIPE_READMODE_MESSAGE: DWORD = 0x00000002;
const PIPE_WAIT: DWORD = 0x00000000;
const PIPE_UNLIMITED_INSTANCES: DWORD = 255;

extern "system" {
    fn CreateNamedPipeW(
        lpName: LPCWSTR,
        dwOpenMode: DWORD,
        dwPipeMode: DWORD,
        nMaxInstances: DWORD,
        nOutBufferSize: DWORD,
        nInBufferSize: DWORD,
        nDefaultTimeOut: DWORD,
        lpSecurityAttributes: LPVOID,
    ) -> HANDLE;

    fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: LPVOID) -> BOOL;
    fn GetLastError() -> DWORD;
    fn CloseHandle(hObject: HANDLE) -> BOOL;
}

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
                        match ConnectNamedPipe(handle, std::ptr::null_mut()) {
                            0 => {
                                let error = GetLastError();
                                if error != ERROR_PIPE_CONNECTED {
                                    CloseHandle(handle);
                                    tx.send(Message::new(
                                        0,
                                        Event::Local(LocalMessage::Error(Box::new(
                                            io::Error::from_raw_os_error(error as _),
                                        ))),
                                    ))
                                    .ok();
                                    continue;
                                }
                            }
                            _ => {}
                        }

                        let client_id = next_client_id.fetch_add(1, Ordering::SeqCst);
                        let mut client = PipeClient::new(
                            client_id,
                            File::from_raw_handle(handle as _),
                            tx.clone(),
                        );
                        client.start_read_thread().ok();
                        clients.lock().unwrap().insert(client_id, client);
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
            if let Err(_) = client.write(data) {
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

    fn disconnect(&mut self, client_id: u32) -> io::Result<()> {
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
                PIPE_ACCESS_DUPLEX,
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
