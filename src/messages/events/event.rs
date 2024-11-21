use super::{client::ClientEvent, local::LocalEvent};

#[derive(Debug)]
pub enum Event {
    Client(ClientEvent),
    Local(LocalEvent),
}

impl Event {
    pub fn on_event(self) {
        match self {
            Event::Client(client_event) => client_event.on_event(),
            Event::Local(local_event) => local_event.on_event(),
        }
    }
}
