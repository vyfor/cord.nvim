pub mod log;
pub mod ready;

pub use log::LogEvent;
pub use ready::ReadyEvent;

use crate::ipc::pipe::PipeServerImpl;

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
}

impl ServerEvent {
    pub fn on_event<T: PipeServerImpl>(self, pipe: &T) {
        let message = match self {
            ServerEvent::Ready(ready_event) => ready_event.on_ready(),
            ServerEvent::Log(log_event) => log_event.on_log(),
        };

        if let Some((id, message)) = message {
            match id {
                0 => pipe.broadcast(message.as_bytes()),
                _ => pipe.write_to(id, message.as_bytes()),
            }
            .ok();
        }
    }
}
