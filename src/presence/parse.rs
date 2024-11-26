use crate::{
    get_field,
    msgpack::{Deserialize, Value},
    remove_field,
};

use super::activity::ActivityContext;

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

        Ok(ActivityContext {
            filename,
            filetype,
            is_read_only,
            cursor_position,
            problem_count,
            custom_asset: None, // todo
            resolved_type: None,
        })
    }
}
