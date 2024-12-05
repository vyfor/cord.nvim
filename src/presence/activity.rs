use crate::{
    get_field_or_none,
    protocol::{
        json,
        msgpack::{self, Value},
    },
    remove_field, remove_field_or_none,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Activity {
    pub details: Option<String>,
    pub state: Option<String>,
    pub assets: Option<ActivityAssets>,
    pub timestamps: Option<ActivityTimestamps>,
    pub buttons: Vec<ActivityButton>,
    pub is_idle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityTimestamps {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
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
        if let Some(assets) = &self.assets {
            f("assets", json::ValueRef::Object(assets), state)?;
        }
        if let Some(timestamps) = &self.timestamps {
            f("timestamps", json::ValueRef::Object(timestamps), state)?;
        }
        if !self.buttons.is_empty() {
            f(
                "buttons",
                json::ValueRef::Array(
                    self.buttons
                        .iter()
                        .map(|b| json::ValueRef::Object(b as &dyn json::Serialize))
                        .collect(),
                ),
                state,
            )?;
        }

        Ok(())
    }
}

impl json::Serialize for ActivityAssets {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
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

        Ok(())
    }
}

impl json::Serialize for ActivityTimestamps {
    fn serialize<'a>(
        &'a self,
        f: json::SerializeFn<'a>,
        state: &mut json::SerializeState,
    ) -> crate::Result<()> {
        if let Some(start) = self.start {
            f("start", json::ValueRef::Number(start as f64), state)?;
        }
        if let Some(end) = self.end {
            f("end", json::ValueRef::Number(end as f64), state)?;
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

impl msgpack::Deserialize for Activity {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid activity")?;

        let details = remove_field_or_none!(input, "details", |v| v.take_string());
        let state = remove_field_or_none!(input, "state", |v| v.take_string());
        let assets =
            remove_field_or_none!(input, "assets", |v| ActivityAssets::deserialize(v).ok());
        let timestamps =
            remove_field_or_none!(input, "timestamps", |v| ActivityTimestamps::deserialize(v)
                .ok());
        let buttons = remove_field_or_none!(input, "buttons", |v| v.take_array().map(|v| v
            .into_iter()
            .map(ActivityButton::deserialize)
            .filter_map(Result::ok)
            .collect()))
        .unwrap_or_default();
        let is_idle = get_field_or_none!(input, "is_idle", |v| v.as_bool()).unwrap_or_default();

        Ok(Activity {
            details,
            state,
            assets,
            timestamps,
            buttons,
            is_idle,
        })
    }
}

impl msgpack::Deserialize for ActivityAssets {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid activity assets")?;

        let large_image = remove_field_or_none!(input, "large_image", |v| v.take_string());
        let large_text = remove_field_or_none!(input, "large_text", |v| v.take_string());
        let small_image = remove_field_or_none!(input, "small_image", |v| v.take_string());
        let small_text = remove_field_or_none!(input, "small_text", |v| v.take_string());

        Ok(ActivityAssets {
            large_image,
            large_text,
            small_image,
            small_text,
        })
    }
}

impl msgpack::Deserialize for ActivityTimestamps {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid activity timestamps")?;

        let start = remove_field_or_none!(input, "start", |v| v.as_uinteger());
        let end = remove_field_or_none!(input, "end", |v| v.as_uinteger());

        Ok(ActivityTimestamps { start, end })
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
