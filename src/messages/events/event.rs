use crate::cord::Cord;

use super::{client::ClientEvent, local::LocalEvent, server::ServerEvent};

#[derive(Debug)]
pub enum Event {
    Client(ClientEvent),
    Local(LocalEvent),
    Server(ServerEvent),
}

pub struct EventContext<'a> {
    pub cord: &'a mut Cord,
    pub client_id: u32,
}

pub trait OnEvent {
    fn on_event(self, ctx: &EventContext) -> crate::Result<()>;
}

impl OnEvent for Event {
    fn on_event(self, ctx: &EventContext) -> crate::Result<()> {
        match self {
            Event::Client(e) => e.on_event(ctx),
            Event::Local(e) => e.on_event(ctx),
            Event::Server(e) => e.on_event(ctx),
        }
    }
}
