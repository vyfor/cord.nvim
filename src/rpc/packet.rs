pub use crate::rpc::activity::Activity;

pub struct Packet {
    pub pid: u32,
    pub activity: Option<Activity>,
}
