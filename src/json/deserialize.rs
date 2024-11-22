use std::collections::HashMap;
use std::str::from_utf8_unchecked;

use super::Json;

pub trait Deserialize: Sized {
    fn deserialize<'a>(input: &HashMap<&'a str, DValue<'a>>) -> crate::Result<Self>;
}

#[derive(Debug)]
pub enum DValue<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Null,
    Array(Vec<DValue<'a>>),
    Object(HashMap<&'a str, DValue<'a>>),
}

impl<'a> DValue<'a> {
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DValue::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<String> {
        match self {
            DValue::String(s) => Some(s.to_string()),
            _ => None,
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<&[DValue]> {
        match self {
            DValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    #[inline]
    pub fn as_map(&self) -> Option<&HashMap<&str, DValue>> {
        match self {
            DValue::Object(map) => Some(map),
            _ => None,
        }
    }

    #[inline]
    pub fn as_number(&self) -> Option<f64> {
        match self {
            DValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, DValue::Null)
    }
}

impl Json {
    pub fn deserialize(input: &str) -> Result<HashMap<&str, DValue>, String> {
        let input = input.trim().as_bytes();
        if input.is_empty() || input[0] != b'{' || input[input.len() - 1] != b'}' {
            return Err("Invalid JSON object".to_string());
        }

        let mut result = HashMap::new();
        let mut pos = 1;
        let len = input.len() - 1;

        while pos < len {
            pos = Self::skip_whitespace(input, pos);
            if pos >= len {
                break;
            }

            if input[pos] != b'"' {
                return Err(format!("Expected '\"', found '{}'", input[pos] as char));
            }
            let (key, new_pos) = Self::parse_string_slice(input, pos + 1)?;
            pos = new_pos;

            pos = Self::skip_whitespace(input, pos);
            if pos >= len || input[pos] != b':' {
                return Err("Expected ':'".to_string());
            }
            pos += 1;

            let (value, new_pos) = Self::parse_value(input, pos)?;
            pos = new_pos;

            result.insert(key, value);

            pos = Self::skip_whitespace(input, pos);
            if pos >= len {
                break;
            }
            if input[pos] == b',' {
                pos += 1;
            }
        }

        Ok(result)
    }

    #[inline]
    fn skip_whitespace(input: &[u8], mut pos: usize) -> usize {
        while pos < input.len() && input[pos].is_ascii_whitespace() {
            pos += 1;
        }
        pos
    }

    fn parse_string_slice(input: &[u8], start: usize) -> Result<(&str, usize), String> {
        let mut pos = start;
        let mut escaped = false;

        while pos < input.len() {
            let c = input[pos];
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'"' {
                let slice = unsafe { from_utf8_unchecked(&input[start..pos]) };
                return Ok((slice, pos + 1));
            }
            pos += 1;
        }
        Err("Unterminated string".to_string())
    }

    fn parse_value(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        let pos = Self::skip_whitespace(input, start);
        if pos >= input.len() {
            return Err("Unexpected end of input".to_string());
        }

        match input[pos] {
            b'"' => {
                let (s, new_pos) = Self::parse_string_slice(input, pos + 1)?;
                Ok((DValue::String(s), new_pos))
            }
            b'[' => Self::parse_array(input, pos + 1),
            b'{' => Self::parse_object(input, pos + 1),
            b't' => Self::parse_true(input, pos),
            b'f' => Self::parse_false(input, pos),
            b'n' => Self::parse_null(input, pos),
            b'-' | b'0'..=b'9' => Self::parse_number(input, pos),
            _ => Err(format!("Unexpected character: '{}'", input[pos] as char)),
        }
    }

    fn parse_array(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        let mut pos = start;
        let mut values = Vec::new();
        let mut expecting_value = true;

        while pos < input.len() {
            pos = Self::skip_whitespace(input, pos);
            if pos >= input.len() {
                return Err("Unterminated array".to_string());
            }

            match input[pos] {
                b']' if !expecting_value => return Ok((DValue::Array(values), pos + 1)),
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
                _ => return Err("Invalid array format".to_string()),
            }
        }
        Err("Unterminated array".to_string())
    }

    fn parse_object(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        let mut pos = start;
        let mut map = HashMap::new();
        let mut expecting_key = true;

        while pos < input.len() {
            pos = Self::skip_whitespace(input, pos);
            if pos >= input.len() {
                return Err("Unterminated object".to_string());
            }

            match input[pos] {
                b'}' if !expecting_key => return Ok((DValue::Object(map), pos + 1)),
                b',' if !expecting_key => {
                    pos += 1;
                    expecting_key = true;
                }
                b'"' if expecting_key => {
                    let (key, new_pos) = Self::parse_string_slice(input, pos + 1)?;
                    pos = Self::skip_whitespace(input, new_pos);

                    if pos >= input.len() || input[pos] != b':' {
                        return Err("Expected ':'".to_string());
                    }
                    pos += 1;

                    let (value, new_pos) = Self::parse_value(input, pos)?;
                    map.insert(key, value);
                    pos = new_pos;
                    expecting_key = false;
                }
                _ => return Err("Invalid object format".to_string()),
            }
        }
        Err("Unterminated object".to_string())
    }

    fn parse_true(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        if input.len() >= start + 4 && &input[start..start + 4] == b"true" {
            Ok((DValue::Bool(true), start + 4))
        } else {
            Err("Invalid 'true' value".to_string())
        }
    }

    fn parse_false(input: &[u8], start: usize) -> Result<(DValue, usize), String> {
        if input.len() >= start + 5 && &input[start..start + 5] == b"false" {
            Ok((DValue::Bool(false), start + 5))
        } else {
            Err("Invalid 'false' value".to_string())
        }
    }

    fn parse_null(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        if input.len() >= start + 4 && &input[start..start + 4] == b"null" {
            Ok((DValue::Null, start + 4))
        } else {
            Err("Invalid 'null' value".to_string())
        }
    }

    fn parse_number(input: &[u8], start: usize) -> Result<(DValue<'_>, usize), String> {
        let mut pos = start;
        let mut num_str = Vec::new();

        if input[pos] == b'-' {
            num_str.push(b'-');
            pos += 1;
        }

        while pos < input.len() && input[pos].is_ascii_digit() {
            num_str.push(input[pos]);
            pos += 1;
        }

        if pos < input.len() && input[pos] == b'.' {
            num_str.push(b'.');
            pos += 1;
            let mut has_decimal = false;
            while pos < input.len() && input[pos].is_ascii_digit() {
                num_str.push(input[pos]);
                has_decimal = true;
                pos += 1;
            }
            if !has_decimal {
                return Err("Invalid number format".to_string());
            }
        }

        if pos < input.len() && (input[pos] == b'e' || input[pos] == b'E') {
            num_str.push(b'e');
            pos += 1;
            if pos < input.len() && (input[pos] == b'+' || input[pos] == b'-') {
                num_str.push(input[pos]);
                pos += 1;
            }
            let mut has_exp = false;
            while pos < input.len() && input[pos].is_ascii_digit() {
                num_str.push(input[pos]);
                has_exp = true;
                pos += 1;
            }
            if !has_exp {
                return Err("Invalid number format".to_string());
            }
        }

        let num_str = unsafe { from_utf8_unchecked(&num_str) };
        match num_str.parse::<f64>() {
            Ok(num) => Ok((DValue::Number(num), pos)),
            Err(_) => Err("Invalid number format".to_string()),
        }
    }
}
