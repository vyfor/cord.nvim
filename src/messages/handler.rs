use std::sync::mpsc::Receiver;

use super::message::Message;

pub struct MessageHandler {
    rx: Receiver<Message>,
}

impl MessageHandler {
    pub fn new(rx: Receiver<Message>) -> Self {
        Self { rx }
    }

    pub fn run(&mut self) {
        for msg in self.rx.iter() {
            msg.event.on_event();
        }
    }
}
