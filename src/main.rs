use std::env::args;

use cord::Cord;

mod cord;
mod ipc;
mod json;
mod mappings;
mod messages;
mod presence;
mod types;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = args().nth(1).ok_or("Missing client ID")?.parse::<u64>()?;

    Cord::new("cord-ipc", client_id)?.run().map_err(Into::into)
}
