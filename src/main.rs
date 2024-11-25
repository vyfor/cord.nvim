use std::env::args;

use cord::{Config, Cord};

mod cord;
mod error;
mod ipc;
mod json;
mod mappings;
mod messages;
mod msgpack;
mod presence;
mod types;
mod util;

use error::Result;

fn main() -> Result<()> {
    let client_id = args().nth(1).ok_or("Missing client ID")?.parse::<u64>()?;

    Cord::new(Config::new("cord-ipc".to_string(), client_id, 30000))?.run()
}
