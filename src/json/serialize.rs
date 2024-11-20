use crate::activity::types::{Activity, Packet};

use std::fmt::{Error, Write};

use super::utils::escape_json;

impl Packet {
    pub fn new(pid: u32, activity: Option<Activity>) -> Packet {
        Packet { pid, activity }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        let mut json_str = String::new();

        json_str.push_str("{\"cmd\":\"SET_ACTIVITY\"");
        json_str.push_str(",\"nonce\":\"-\"");
        json_str.push_str(",\"args\":{");

        write!(&mut json_str, "\"pid\":{}", self.pid)?;
        if let Some(activity) = &self.activity {
            json_str.push_str(",\"activity\":");
            activity.push_json(&mut json_str)?;
        }

        json_str.push_str("}}");

        Ok(json_str)
    }
}

impl Activity {
    pub fn push_json(&self, json_str: &mut String) -> Result<(), Error> {
        json_str.push_str("{\"type\":0");

        if let Some(timestamp) = &self.timestamp {
            write!(json_str, ",\"timestamps\":{{\"start\":{}}}", timestamp)?;
        }

        if let Some(details) = &self.details {
            write!(json_str, ",\"details\":\"{}\"", escape_json(details))?;
        }

        if let Some(state) = &self.state {
            write!(json_str, ",\"state\":\"{}\"", escape_json(state))?;
        }

        if self.large_image.is_some()
            || self.large_text.is_some()
            || self.small_image.is_some()
            || self.small_text.is_some()
        {
            json_str.push_str(",\"assets\":{");

            if let Some(large_image) = &self.large_image {
                write!(json_str, "\"large_image\":\"{}\",", large_image)?;
            }

            if let Some(large_text) = &self.large_text {
                write!(json_str, "\"large_text\":\"{}\",", escape_json(large_text))?;
            }

            if let Some(small_image) = &self.small_image {
                write!(json_str, "\"small_image\":\"{}\",", small_image)?;
            }

            if let Some(small_text) = &self.small_text {
                write!(json_str, "\"small_text\":\"{}\"", escape_json(small_text))?;
            }

            if json_str.ends_with(',') {
                json_str.pop();
            }

            json_str.push('}');
        }

        if let Some(buttons) = &self.buttons {
            json_str.push_str(",\"buttons\":[");

            for (index, button) in buttons.iter().enumerate() {
                if index > 0 {
                    json_str.push(',');
                }
                write!(
                    json_str,
                    "{{\"label\":\"{}\",\"url\":\"{}\"}}",
                    escape_json(&button.label),
                    button.url
                )?;
            }

            json_str.push(']');
        }

        json_str.push('}');

        Ok(())
    }
}
