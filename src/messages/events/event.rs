use super::{client::ClientEvent, local::LocalEvent};

#[derive(Debug)]
pub enum Event {
    Client(ClientEvent),
    Local(LocalEvent),
}
