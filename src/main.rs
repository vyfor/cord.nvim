use std::env::args;

use cord::Cord;

mod cord;
mod error;
mod ipc;
mod json;
mod mappings;
mod messages;
mod presence;
mod types;
mod util;

use error::Result;

fn main() -> Result<()> {
    let client_id = args().nth(1).ok_or("Missing client ID")?.parse::<u64>()?;

    Cord::new("cord-ipc", client_id, 30)?.run()
}
