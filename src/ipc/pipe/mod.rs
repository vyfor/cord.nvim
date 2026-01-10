pub mod platform;

use std::io;
use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::messages::events::local::ErrorEvent;
use crate::messages::message::Message;
use crate::session::SessionManager;
use crate::{client_event, debug, local_event};

/// Trait for server-side pipe operations.
///
/// This trait defines methods for managing a pipe server, including starting,
/// stopping, broadcasting, and writing to clients.
pub trait PipeServerImpl {
    /// Creates a new pipe server instance.
    ///
    /// # Arguments
    ///
    /// * `pipe_name` - The name of the pipe.
    /// * `tx` - A channel sender for sending messages.
    /// * `session_manager` - A session manager for handling client sessions.
    fn new(
        pipe_name: &str,
        tx: Sender<Message>,
        session_manager: Arc<SessionManager>,
    ) -> Self
    where
        Self: Sized;

    /// Starts the pipe server.
    fn start(&mut self) -> io::Result<()>;

    /// Stops the pipe server.
    fn stop(&mut self);

    /// Broadcasts data to all connected clients.
    fn broadcast(&self, data: &[u8]) -> io::Result<()>;

    /// Writes data to a specific client.
    fn write_to(&self, client_id: u32, data: &[u8]) -> io::Result<()>;

    /// Disconnects a specific client.
    #[allow(dead_code)]
    fn disconnect(&self, client_id: u32) -> io::Result<()>;
}

/// Trait for client-side pipe operations.
///
/// This trait defines methods for managing a pipe client, including writing data
/// and starting a read thread.
pub trait PipeClientImpl {
    /// Creates a new pipe client instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the client.
    /// * `pipe` - The pipe type used for communication.
    /// * `tx` - A channel sender for sending messages.
    fn new(id: u32, pipe: Self::PipeType, tx: Sender<Message>) -> Self
    where
        Self: Sized;

    /// Writes data to the pipe.
    fn write(&mut self, data: &[u8]) -> io::Result<()>;

    /// Starts a thread for reading data from the pipe.
    fn start_read_thread(&mut self) -> io::Result<()>;

    /// The type of pipe used for communication.
    type PipeType;
}

/// Handles error reporting for a specific client.
///
/// Sends an error event message when an error occurs, managing broken pipe errors
/// by sending a disconnect event.
fn report_error(id: u32, tx: &Sender<Message>, error: io::Error) {
    if error.kind() == io::ErrorKind::BrokenPipe
        || error.kind() == io::ErrorKind::ConnectionReset
    {
        debug!("Client {} disconnected: {}", id, error);
        tx.send(client_event!(id, Disconnect)).ok();
        return;
    }

    debug!("Error for client {}: {}", id, error);
    tx.send(local_event!(0, Error, ErrorEvent::new(Box::new(error))))
        .ok();
}
