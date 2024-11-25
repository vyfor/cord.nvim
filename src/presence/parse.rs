use crate::{
    get_field,
    msgpack::{Deserialize, Value},
    remove_field,
    util::types::AssetType,
};

use super::activity::{ActivityContext, CustomAssetContext};

impl Deserialize for ActivityContext {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid activity context")?;

        let filename = remove_field!(input, "filename", |v| v.take_string());
        let filetype = remove_field!(input, "filetype", |v| v.take_string());
        let is_read_only = get_field!(input, "is_read_only", |v| v.as_bool());
        let cursor_position = input
            .get("cursor_position")
            .and_then(|cursor| cursor.as_array())
            .and_then(|array| {
                array.first().and_then(|v| v.as_uinteger()).map(|line| {
                    array
                        .get(1)
                        .and_then(|v| v.as_uinteger())
                        .map(|char| (line as u32, char as u32))
                })
            })
            .flatten();
        let problem_count = get_field!(input, "problem_count", |v| v.as_integer()) as i32;
        let custom_asset = input
            .remove("custom_asset")
            .and_then(|v| CustomAssetContext::deserialize(v).ok());

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
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid custom asset context")?;

        let ty = get_field!(input, "type", |v| v.as_str().and_then(AssetType::from));
        let name = remove_field!(input, "name", |v| v.take_string());
        let icon = remove_field!(input, "icon", |v| v.take_string());
        let tooltip = remove_field!(input, "tooltip", |v| v.take_string());

        Ok(CustomAssetContext {
            ty,
            name,
            icon,
            tooltip,
        })
    }
}
