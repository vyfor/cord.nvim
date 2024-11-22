use std::collections::HashMap;
use std::io;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::ipc::pipe::{PipeClientImpl, PipeServerImpl};
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::{client_event, local_event};

use super::client::PipeClient;

pub struct PipeServer {
    pipe_name: String,
    tx: Sender<Message>,
    clients: Arc<Mutex<HashMap<u32, PipeClient>>>,
    next_client_id: Arc<AtomicU32>,
    running: Arc<AtomicBool>,
    listener: Option<UnixListener>,
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
            listener: None,
            thread_handle: None,
        }
    }

    fn start(&mut self) -> io::Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        if Path::new(&self.pipe_name).exists() {
            std::fs::remove_file(&self.pipe_name)?;
        }

        let listener = UnixListener::bind(&self.pipe_name)?;
        self.listener = Some(listener);
        self.running.store(true, Ordering::SeqCst);

        let clients = Arc::clone(&self.clients);
        let next_client_id = Arc::clone(&self.next_client_id);
        let listener = self.listener.as_ref().unwrap().try_clone()?;
        let running = Arc::clone(&self.running);
        let tx = self.tx.clone();

        self.thread_handle = Some(std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let client_id = next_client_id.fetch_add(1, Ordering::SeqCst);
                        let mut client = PipeClient::new(client_id, stream, tx.clone());
                        tx.send(client_event!(0, Connect)).ok();
                        client.start_read_thread().ok();
                        clients.lock().unwrap().insert(client_id, client);
                    }
                    Err(e) => {
                        tx.send(local_event!(0, Error, ErrorEvent::new(Box::new(e))))
                            .ok();
                    }
                }
            }
        }));

        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(listener) = self.listener.take() {
            drop(listener);
        }
        if Path::new(&self.pipe_name).exists() {
            let _ = std::fs::remove_file(&self.pipe_name);
        }
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

impl Drop for PipeServer {
    fn drop(&mut self) {
        self.stop();
    }
}
