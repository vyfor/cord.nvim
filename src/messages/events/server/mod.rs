pub mod log;
pub mod ready;

pub use log::LogEvent;
pub use ready::ReadyEvent;

#[derive(Debug)]
pub enum ServerEvent {
    Ready(ReadyEvent),
    Log(LogEvent),
}

impl ServerEvent {
    pub fn on_event(self) {
        match self {
            ServerEvent::Ready(ready_event) => ready_event.on_ready(),
            ServerEvent::Log(log_event) => log_event.on_log(),
        }
    }
}
