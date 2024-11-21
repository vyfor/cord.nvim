use super::events::event::Event;

#[derive(Debug)]
pub struct Message {
    pub client_id: u32,
    pub event: Event,
}

impl Message {
    pub fn new(client_id: u32, message: Event) -> Self {
        Self {
            client_id,
            event: message,
        }
    }
}

#[macro_export]
macro_rules! client_event {
    ($id:expr, $type:ident, $args:expr) => {
        $crate::messages::message::Message::new(
            $id,
            $crate::messages::events::event::Event::Client(
                $crate::messages::events::client::ClientEvent::$type($args),
            ),
        )
    };

    ($id:expr, $type:ident) => {
        $crate::messages::message::Message::new(
            $id,
            $crate::messages::events::event::Event::Client(
                $crate::messages::events::client::ClientEvent::$type(Default::default()),
            ),
        )
    };
}

#[macro_export]
macro_rules! local_event {
    ($id:expr, $type:ident, $args:expr) => {
        $crate::messages::message::Message::new(
            $id,
            $crate::messages::events::event::Event::Local(
                $crate::messages::events::local::LocalEvent::$type($args),
            ),
        )
    };

    ($id:expr, $type:ident) => {
        $crate::messages::message::Message::new(
            $id,
            $crate::messages::events::event::Event::Local(
                $crate::messages::events::local::LocalEvent::$type(Default::default()),
            ),
        )
    };
}
