use std::collections::HashMap;

use crate::{
    json::deserialize::{DValue, Deserialize},
    util::types::AssetType,
};

use super::activity::{ActivityContext, CustomAssetContext};

impl Deserialize for ActivityContext {
    fn deserialize<'a>(input: &HashMap<&'a str, DValue<'a>>) -> crate::Result<Self> {
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
            .and_then(|v| v.ok());

        Ok(ActivityContext {
            filename,
            filetype,
            custom_asset,
            resolved_type: None,
        })
    }
}

impl Deserialize for CustomAssetContext {
    fn deserialize<'a>(input: &HashMap<&'a str, DValue<'a>>) -> crate::Result<Self> {
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
