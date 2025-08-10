use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod lockfile;
pub mod logger;
pub mod macros;

pub fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}
