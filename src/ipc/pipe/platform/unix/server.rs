use std::io;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use super::client::PipeClient;
use crate::ipc::pipe::{PipeClientImpl, PipeServerImpl};
use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::session::SessionManager;
use crate::{client_event, echoln, local_event};

pub struct PipeServer {
    session_manager: Arc<SessionManager>,
    pipe_name: String,
    tx: Sender<Message>,
    next_client_id: Arc<AtomicU32>,
    running: Arc<AtomicBool>,
    listener: Option<UnixListener>,
    thread_handle: Option<JoinHandle<()>>,
}

impl PipeServerImpl for PipeServer {
    fn new(
        pipe_name: &str,
        tx: Sender<Message>,
        session_manager: Arc<SessionManager>,
    ) -> Self {
        Self {
            session_manager,
            pipe_name: pipe_name.to_string(),
            tx,
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

        let tx = self.tx.clone();
        let session_manager = Arc::clone(&self.session_manager);
        let next_client_id = Arc::clone(&self.next_client_id);
        let running = Arc::clone(&self.running);
        let listener = self.listener.as_ref().unwrap().try_clone()?;

        self.thread_handle = Some(std::thread::spawn(move || {
            let mut notified = false;
            while running.load(Ordering::SeqCst) {
                if !notified {
                    echoln!("Ready");
                    notified = true;
                }

                match listener.accept() {
                    Ok((stream, _)) => {
                        let client_id =
                            next_client_id.fetch_add(1, Ordering::SeqCst);
                        let mut client =
                            PipeClient::new(client_id, stream, tx.clone());
                        client.start_read_thread().ok();
                        session_manager.create_session(client_id, client);
                        tx.send(client_event!(client_id, Connect)).ok();
                    }
                    Err(e) => {
                        tx.send(local_event!(
                            0,
                            Error,
                            ErrorEvent::new(Box::new(e))
                        ))
                        .ok();
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
        if let Some(listener) = self.listener.take() {
            drop(listener);
        }
        let _ = std::fs::remove_file(&self.pipe_name);
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

impl Drop for PipeServer {
    fn drop(&mut self) {
        self.stop();
    }
}
