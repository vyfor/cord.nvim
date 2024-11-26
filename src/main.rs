mod cord;
mod error;
mod ipc;
mod json;
mod mappings;
mod messages;
mod msgpack;
mod presence;
mod session;
mod types;
mod util;

use cord::{Config, Cord};
use error::Result;
use util::utils::parse_client_id;

fn main() -> Result<()> {
    let (client_id, is_custom_client) = parse_client_id();

    Cord::new(Config::new(
        "cord-ipc".to_string(),
        client_id,
        is_custom_client,
        30000,
    ))?
    .run()
}
