use std::sync::mpsc::Receiver;

use crate::ipc::pipe::PipeServerImpl;

use super::message::Message;

pub struct MessageHandler {
    rx: Receiver<Message>,
}

impl MessageHandler {
    pub fn new(rx: Receiver<Message>) -> Self {
        Self { rx }
    }

    pub fn run<T: PipeServerImpl>(&mut self, pipe: &T) {
        for msg in self.rx.iter() {
            msg.event.on_event(pipe);
        }
    }
}
