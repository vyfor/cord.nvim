use crate::{
    msgpack::{Deserialize, MsgPack},
    util::types::AssetType,
};

use super::activity::{ActivityContext, CustomAssetContext};

impl Deserialize for ActivityContext {
    fn deserialize<'a>(input: &[u8]) -> crate::Result<Self> {
        let mut input = MsgPack::deserialize(input)?
            .take_map()
            .ok_or("Invalid activity context")?;

        let filename = input
            .remove("filename")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'filename' field")?;

        let filetype = input
            .remove("filetype")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'filetype' field")?;

        let is_read_only = input
            .get("is_read_only")
            .and_then(|v| v.as_bool())
            .ok_or("Missing or invalid 'is_read_only' field")?;

        let mut cursor_position = None;
        if let Some(cursor) = input.get("cursor_position") {
            let array = cursor.as_array().ok_or("Invalid 'cursor_position' field")?;
            let line = array
                .first()
                .and_then(|v| v.as_uinteger())
                .ok_or("Invalid 'cursor_position' field")? as u32;
            let char = array
                .get(1)
                .and_then(|v| v.as_uinteger())
                .ok_or("Invalid 'cursor_position' field")? as u32;

            cursor_position = Some((line, char));
        }

        let problem_count = input
            .get("problem_count")
            .and_then(|v| v.as_integer())
            .ok_or("Missing or invalid 'problem_count' field")? as i32;

        let custom_asset = input
            .get("custom_asset")
            .and_then(|v| v.as_bytes().map(CustomAssetContext::deserialize))
            .and_then(|v| v.ok());

        Ok(ActivityContext {
            filename,
            filetype,
            is_read_only,
            cursor_position,
            problem_count,
            custom_asset,
            resolved_type: None,
        })
    }
}

impl Deserialize for CustomAssetContext {
    fn deserialize<'a>(input: &[u8]) -> crate::Result<Self> {
        let mut input = MsgPack::deserialize(input)?
            .take_map()
            .ok_or("Invalid custom asset context")?;

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
            .remove("icon")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'icon' field")?;

        let tooltip = input
            .remove("tooltip")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'tooltip' field")?;

        Ok(CustomAssetContext {
            ty,
            name,
            icon,
            tooltip,
        })
    }
}
