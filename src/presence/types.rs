use crate::{
    json,
    msgpack::{self, Value},
    remove_field,
};

pub struct Packet<'a> {
    pub pid: u32,
    pub activity: Option<&'a Activity>,
}

impl<'a> Packet<'a> {
    pub fn new(pid: u32, activity: Option<&'a Activity>) -> Self {
        Self { pid, activity }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Activity {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub buttons: Option<Vec<ActivityButton>>,
    pub timestamp: Option<u128>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

impl json::Serialize for Packet<'_> {
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

impl json::Serialize for Activity {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
        if let Some(details) = &self.details {
            f("details", json::ValueRef::String(details), state)?;
        }
        if let Some(state_str) = &self.state {
            f("state", json::ValueRef::String(state_str), state)?;
        }
        if let Some(large_image) = &self.large_image {
            f("large_image", json::ValueRef::String(large_image), state)?;
        }
        if let Some(large_text) = &self.large_text {
            f("large_text", json::ValueRef::String(large_text), state)?;
        }
        if let Some(small_image) = &self.small_image {
            f("small_image", json::ValueRef::String(small_image), state)?;
        }
        if let Some(small_text) = &self.small_text {
            f("small_text", json::ValueRef::String(small_text), state)?;
        }
        if let Some(buttons) = &self.buttons {
            if !buttons.is_empty() {
                f(
                    "buttons",
                    json::ValueRef::Array(
                        buttons
                            .iter()
                            .map(|b| json::ValueRef::Object(b as &dyn json::Serialize))
                            .collect(),
                    ),
                    state,
                )?;
            }
        }
        if let Some(timestamp) = &self.timestamp {
            f(
                "timestamp",
                json::ValueRef::Number(*timestamp as f64),
                state,
            )?;
        }
        Ok(())
    }
}

impl json::Serialize for ActivityButton {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
        f("label", json::ValueRef::String(&self.label), state)?;
        f("url", json::ValueRef::String(&self.url), state)?;
        Ok(())
    }
}

impl msgpack::Deserialize for ActivityButton {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid activity button")?;

        let label = remove_field!(input, "label", |v| v.take_string());
        let url = remove_field!(input, "url", |v| v.take_string());

        Ok(ActivityButton { label, url })
    }
}
