use crate::protocol::json;

use super::activity::Activity;

pub struct Packet<'a> {
    pub cmd: &'a str,
    pub args: PacketArgs<'a>,
}

pub struct PacketArgs<'a> {
    pub pid: u32,
    pub activity: Option<&'a Activity>,
}

impl<'a> Packet<'a> {
    pub fn new(pid: u32, activity: Option<&'a Activity>) -> Self {
        Self {
            cmd: "SET_ACTIVITY",
            args: PacketArgs { pid, activity },
        }
    }

    pub fn empty() -> Self {
        Self {
            cmd: "SET_ACTIVITY",
            args: PacketArgs {
                pid: 0,
                activity: None,
            },
        }
    }
}

impl json::Serialize for Packet<'_> {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
        f("cmd", json::ValueRef::String(self.cmd), state)?;
        f("args", json::ValueRef::Object(&self.args), state)?;
        f("nonce", json::ValueRef::String("-"), state)?;

        Ok(())
    }
}

impl json::Serialize for PacketArgs<'_> {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
        f("pid", json::ValueRef::Number(self.pid as f64), state)?;
        if let Some(activity) = &self.activity {
            f("activity", json::ValueRef::Object(*activity), state)?;
        }

        Ok(())
    }
}
