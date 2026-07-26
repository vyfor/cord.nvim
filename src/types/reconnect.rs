use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct ReconnectState {
    pub in_progress: bool,
    pub cancel: Option<Arc<AtomicBool>>,
}

impl ReconnectState {
    pub fn cancel(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.store(true, Ordering::SeqCst);
        }
        self.in_progress = false;
    }
}
