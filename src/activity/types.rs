use crate::json::parser::Deserializable;

pub struct Packet {
    pub pid: u32,
    pub activity: Option<Activity>,
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

impl Deserializable for ActivityButton {
    fn deserialize(
        input: &std::collections::HashMap<String, crate::json::parser::Value>,
    ) -> Result<Self, String> {
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
