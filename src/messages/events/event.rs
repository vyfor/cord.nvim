use std::sync::Arc;

use crate::ipc::{discord::client::RichClient, pipe::platform::server::PipeServer};

use super::{client::ClientEvent, local::LocalEvent, server::ServerEvent};

#[derive(Debug)]
pub enum Event {
    Client(ClientEvent),
    Local(LocalEvent),
    Server(ServerEvent),
}

pub struct EventContext<'a> {
    pub pipe: &'a PipeServer,
    pub rich_client: Arc<RichClient>,
}

pub trait OnEvent {
    fn on_event(self, ctx: &EventContext);
}

impl OnEvent for Event {
    fn on_event(self, ctx: &EventContext) {
        match self {
            Event::Client(e) => e.on_event(ctx),
            Event::Local(e) => e.on_event(ctx),
            Event::Server(e) => e.on_event(ctx),
        }
    }
}
