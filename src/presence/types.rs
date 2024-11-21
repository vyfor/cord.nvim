use std::collections::HashMap;

use crate::json::{
    deserialize::{DValue, Deserializable},
    serialize::{SValue, Serialize, SerializeFn, SerializeState},
};

pub struct Packet {
    pub pid: u32,
    pub activity: Option<Activity>,
}

impl Serialize for Packet {
    fn serialize<'a>(
        &'a self,
        f: SerializeFn<'a>,
        state: &mut SerializeState,
    ) -> Result<(), String> {
        f("pid", SValue::Number(self.pid as f64), state)?;
        if let Some(activity) = &self.activity {
            f("activity", SValue::Object(activity), state)?;
        }
        Ok(())
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

impl Serialize for Activity {
    fn serialize<'a>(
        &'a self,
        f: SerializeFn<'a>,
        state: &mut SerializeState,
    ) -> Result<(), String> {
        if let Some(details) = &self.details {
            f("details", SValue::String(details), state)?;
        }
        if let Some(state_str) = &self.state {
            f("state", SValue::String(state_str), state)?;
        }
        if let Some(large_image) = &self.large_image {
            f("large_image", SValue::String(large_image), state)?;
        }
        if let Some(large_text) = &self.large_text {
            f("large_text", SValue::String(large_text), state)?;
        }
        if let Some(small_image) = &self.small_image {
            f("small_image", SValue::String(small_image), state)?;
        }
        if let Some(small_text) = &self.small_text {
            f("small_text", SValue::String(small_text), state)?;
        }
        if let Some(buttons) = &self.buttons {
            if !buttons.is_empty() {
                f(
                    "buttons",
                    SValue::Array(
                        buttons
                            .iter()
                            .map(|b| SValue::Object(b as &dyn Serialize))
                            .collect(),
                    ),
                    state,
                )?;
            }
        }
        if let Some(timestamp) = &self.timestamp {
            f("timestamp", SValue::Number(*timestamp as f64), state)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

impl Serialize for ActivityButton {
    fn serialize<'a>(
        &'a self,
        f: SerializeFn<'a>,
        state: &mut SerializeState,
    ) -> Result<(), String> {
        f("label", SValue::String(&self.label), state)?;
        f("url", SValue::String(&self.url), state)?;
        Ok(())
    }
}

impl Deserializable for ActivityButton {
    fn deserialize(input: &HashMap<String, DValue>) -> Result<Self, String> {
        let label = input
            .get("label")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'label' field")?
            .to_string();
        let url = input
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'url' field")?
            .to_string();
        Ok(ActivityButton { label, url })
    }
}
