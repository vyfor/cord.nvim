use std::collections::HashMap;
use std::str::from_utf8_unchecked;

use super::value::Value;
use super::Json;
use crate::protocol::error::ProtocolError;

/// Trait for deserializing JSON data into Rust types.
///
/// This trait defines a method for converting a JSON representation into
/// a Rust data structure. It requires implementing the `deserialize` method
/// that takes a reference to a `HashMap` and returns the desired type.
pub trait Deserialize: Sized {
    /// Deserializes a JSON object into a Rust type.
    ///
    /// # Arguments
    ///
    /// * `input` - A reference to a `HashMap` containing JSON key-value pairs.
    ///
    /// # Returns
    ///
    /// A result containing the deserialized Rust type or an error.
    fn deserialize<'a>(
        input: &HashMap<&'a str, Value<'a>>,
    ) -> crate::Result<Self>;
}

impl Json {
    pub fn deserialize(input: &str) -> crate::Result<HashMap<&str, Value>> {
        let input = input.trim().as_bytes();
        if input.is_empty()
            || input[0] != b'{'
            || input[input.len() - 1] != b'}'
        {
            return Err(
                ProtocolError::InvalidSyntax("Invalid JSON object").into()
            );
        }

        let (value, _) = Self::parse_object(input, 1)?;
        match value {
            Value::Object(map) => Ok(map),
            _ => Err(ProtocolError::InvalidSyntax("Expected object").into()),
        }
    }

    #[inline]
    fn skip_whitespace(input: &[u8], mut pos: usize) -> usize {
        while pos < input.len() && input[pos].is_ascii_whitespace() {
            pos += 1;
        }
        pos
    }

    fn parse_string_slice(
        input: &[u8],
        mut pos: usize,
    ) -> crate::Result<(&str, usize)> {
        let start = pos;
        while pos < input.len() {
            match input[pos] {
                b'"' => {
                    let s = unsafe { from_utf8_unchecked(&input[start..pos]) };
                    return Ok((s, pos + 1));
                }
                b'\\' => {
                    pos += 2;
                }
                _ => pos += 1,
            }
        }
        Err(ProtocolError::UnexpectedEnd.into())
    }

    fn parse_value(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        let pos = Self::skip_whitespace(input, start);
        if pos >= input.len() {
            return Err(ProtocolError::UnexpectedEnd.into());
        }

        match input[pos] {
            b'"' => {
                let (s, new_pos) = Self::parse_string_slice(input, pos + 1)?;
                Ok((Value::String(s), new_pos))
            }
            b'[' => Self::parse_array(input, pos + 1),
            b'{' => Self::parse_object(input, pos + 1),
            b't' => Self::parse_true(input, pos),
            b'f' => Self::parse_false(input, pos),
            b'n' => Self::parse_null(input, pos),
            b'-' | b'0'..=b'9' => Self::parse_number(input, pos),
            c => Err(ProtocolError::UnexpectedChar(c as char).into()),
        }
    }

    fn parse_array(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        let mut pos = start;
        let mut values = Vec::new();
        let mut expecting_value = true;

        while pos < input.len() {
            pos = Self::skip_whitespace(input, pos);
            if pos >= input.len() {
                return Err(ProtocolError::UnexpectedEnd.into());
            }

            match input[pos] {
                b']' if !expecting_value => {
                    return Ok((Value::Array(values), pos + 1))
                }
                b',' if !expecting_value => {
                    pos += 1;
                    expecting_value = true;
                }
                _ if expecting_value => {
                    let (value, new_pos) = Self::parse_value(input, pos)?;
                    values.push(value);
                    pos = new_pos;
                    expecting_value = false;
                }
                c => {
                    return Err(ProtocolError::UnexpectedChar(c as char).into())
                }
            }
        }
        Err(ProtocolError::UnexpectedEnd.into())
    }

    fn parse_object(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        let mut pos = start;
        let mut map = HashMap::new();
        let mut expecting_key = true;

        while pos < input.len() {
            pos = Self::skip_whitespace(input, pos);
            if pos >= input.len() {
                return Err(ProtocolError::UnexpectedEnd.into());
            }

            match input[pos] {
                b'}' if !expecting_key => {
                    return Ok((Value::Object(map), pos + 1))
                }
                b',' if !expecting_key => {
                    pos += 1;
                    expecting_key = true;
                }
                b'"' if expecting_key => {
                    let (key, new_pos) =
                        Self::parse_string_slice(input, pos + 1)?;
                    pos = Self::skip_whitespace(input, new_pos);
                    if pos >= input.len() || input[pos] != b':' {
                        return Err(ProtocolError::InvalidSyntax(
                            "Expected ':' after key",
                        )
                        .into());
                    }
                    pos = Self::skip_whitespace(input, pos + 1);
                    let (value, new_pos) = Self::parse_value(input, pos)?;
                    map.insert(key, value);
                    pos = new_pos;
                    expecting_key = false;
                }
                c => {
                    return Err(ProtocolError::UnexpectedChar(c as char).into())
                }
            }
        }
        Err(ProtocolError::UnexpectedEnd.into())
    }

    fn parse_true(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        if input.len() >= start + 4 && &input[start..start + 4] == b"true" {
            Ok((Value::Bool(true), start + 4))
        } else {
            Err(ProtocolError::InvalidSyntax("Invalid 'true' value").into())
        }
    }

    fn parse_false(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value, usize)> {
        if input.len() >= start + 5 && &input[start..start + 5] == b"false" {
            Ok((Value::Bool(false), start + 5))
        } else {
            Err(ProtocolError::InvalidSyntax("Invalid 'false' value").into())
        }
    }

    fn parse_null(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        if input.len() >= start + 4 && &input[start..start + 4] == b"null" {
            Ok((Value::Null, start + 4))
        } else {
            Err(ProtocolError::InvalidSyntax("Invalid 'null' value").into())
        }
    }

    fn parse_number(
        input: &[u8],
        start: usize,
    ) -> crate::Result<(Value<'_>, usize)> {
        let mut pos = start;
        let mut num_str = Vec::new();

        if input[pos] == b'-' {
            num_str.push(b'-');
            pos += 1;
        }

        while pos < input.len() {
            match input[pos] {
                b'0'..=b'9' => {
                    num_str.push(input[pos]);
                    pos += 1;
                }
                b'.' => {
                    if num_str.contains(&b'.') {
                        return Err(ProtocolError::InvalidSyntax(
                            "Multiple decimal points",
                        )
                        .into());
                    }
                    num_str.push(b'.');
                    pos += 1;
                }
                b'e' | b'E' => {
                    num_str.push(b'e');
                    pos += 1;
                    if pos < input.len()
                        && (input[pos] == b'+' || input[pos] == b'-')
                    {
                        num_str.push(input[pos]);
                        pos += 1;
                    }
                }
                _ if input[pos].is_ascii_whitespace()
                    || input[pos] == b','
                    || input[pos] == b'}'
                    || input[pos] == b']' =>
                {
                    break;
                }
                c => {
                    return Err(ProtocolError::UnexpectedChar(c as char).into())
                }
            }
        }

        let num_str = unsafe { from_utf8_unchecked(&num_str) };
        match num_str.parse::<f64>() {
            Ok(num) => Ok((Value::Number(num), pos)),
            Err(_) => {
                Err(ProtocolError::InvalidSyntax("Invalid number format")
                    .into())
            }
        }
    }
}
