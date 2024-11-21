use std::collections::HashMap;

use crate::{
    json::deserialize::{DValue, Deserializable},
    util::types::AssetType,
};

use super::activity::{ActivityContext, CustomAssetContext};

impl Deserializable for ActivityContext {
    fn deserialize(input: &HashMap<String, DValue>) -> Result<Self, String> {
        let filename = input
            .get("filename")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'filename' field")?
            .to_string();

        let filetype = input
            .get("filetype")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'filetype' field")?
            .to_string();

        let custom_asset = input
            .get("custom_asset")
            .and_then(|v| v.as_map().map(CustomAssetContext::deserialize))
            .map(|v| v.ok())
            .flatten();

        Ok(ActivityContext {
            filename,
            filetype,
            custom_asset,
            resolved_type: None,
        })
    }
}

impl Deserializable for CustomAssetContext {
    fn deserialize(input: &HashMap<String, DValue>) -> Result<Self, String> {
        let ty = input
            .get("type")
            .and_then(|v| v.as_str().map(AssetType::from))
            .flatten()
            .ok_or("Missing or invalid 'type' field")?;

        let name = input
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'name' field")?
            .to_string();

        let icon = input
            .get("icon")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'icon' field")?
            .to_string();

        let tooltip = input
            .get("tooltip")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'tooltip' field")?
            .to_string();

        Ok(CustomAssetContext {
            ty,
            name,
            icon,
            tooltip,
        })
    }
}
