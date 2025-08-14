use std::collections::HashMap;

use super::value::Value;
use super::{
    ARRAY16, ARRAY32, FALSE, FIXARRAY_MASK, FIXARRAY_SIZE_MASK, FIXARRAY_VALUE,
    FIXMAP_MASK, FIXMAP_SIZE_MASK, FIXMAP_VALUE, FIXSTR_MASK, FIXSTR_SIZE_MASK,
    FIXSTR_VALUE, FLOAT32, FLOAT64, INT8, INT16, INT32, INT64, MAP16, MAP32,
    MsgPack, NEGATIVE_FIXINT_MASK, NEGATIVE_FIXINT_VALUE, NIL,
    POSITIVE_FIXINT_MASK, POSITIVE_FIXINT_VALUE, STR8, STR16, STR32, TRUE,
    UINT8, UINT16, UINT32, UINT64,
};
use crate::protocol::error::ProtocolError;

/// Trait for deserializing MsgPack data into Rust types.
///
/// This trait defines a method for converting a MsgPack representation into
/// a Rust data structure. It requires implementing the `deserialize` method
/// that takes a `Value` and returns the desired type.
pub trait Deserialize: Sized {
    /// Deserializes a MsgPack value into a Rust type.
    ///
    /// # Arguments
    ///
    /// * `input` - A `Value` representing the MsgPack data.
    ///
    /// # Returns
    ///
    /// A result containing the deserialized Rust type or an error.
    fn deserialize(input: Value) -> crate::Result<Self>;
}

impl MsgPack {
    pub fn deserialize(input: &[u8]) -> crate::Result<Value> {
        if input.is_empty() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let mut pos = 0;
        Self::parse_value(input, &mut pos)
    }

    fn parse_value(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos >= input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }

        let byte = input[*pos];
        *pos += 1;

        match byte {
            NIL => Ok(Value::Nil),
            TRUE => Ok(Value::Boolean(true)),
            FALSE => Ok(Value::Boolean(false)),

            b if (b & POSITIVE_FIXINT_MASK) == POSITIVE_FIXINT_VALUE => {
                Ok(Value::Integer(b as i64))
            }
            b if (b & NEGATIVE_FIXINT_MASK) == NEGATIVE_FIXINT_VALUE => {
                Ok(Value::Integer((b as i8) as i64))
            }
            b if (b & FIXSTR_MASK) == FIXSTR_VALUE => {
                let len = (b & FIXSTR_SIZE_MASK) as usize;
                Self::parse_str(input, pos, len)
            }

            STR8 => {
                let len = input[*pos] as usize;
                *pos += 1;
                Self::parse_str(input, pos, len)
            }
            STR16 => {
                let len = Self::parse_u16(input, pos)? as usize;
                Self::parse_str(input, pos, len)
            }
            STR32 => {
                let len = Self::parse_u32(input, pos)? as usize;
                Self::parse_str(input, pos, len)
            }

            b if (b & FIXARRAY_MASK) == FIXARRAY_VALUE => {
                let len = (b & FIXARRAY_SIZE_MASK) as usize;
                Self::parse_array(input, pos, len)
            }

            ARRAY16 => {
                let len = Self::parse_u16(input, pos)? as usize;
                Self::parse_array(input, pos, len)
            }
            ARRAY32 => {
                let len = Self::parse_u32(input, pos)? as usize;
                Self::parse_array(input, pos, len)
            }

            b if (b & FIXMAP_MASK) == FIXMAP_VALUE => {
                let len = (b & FIXMAP_SIZE_MASK) as usize;
                Self::parse_map(input, pos, len)
            }

            MAP16 => {
                let len = Self::parse_u16(input, pos)? as usize;
                Self::parse_map(input, pos, len)
            }
            MAP32 => {
                let len = Self::parse_u32(input, pos)? as usize;
                Self::parse_map(input, pos, len)
            }

            INT8 => Self::parse_i8(input, pos),
            INT16 => Self::parse_i16(input, pos),
            INT32 => Self::parse_i32(input, pos),
            INT64 => Self::parse_i64(input, pos),

            UINT8 => Self::parse_u8(input, pos),
            UINT16 => Self::parse_u16_value(input, pos),
            UINT32 => Self::parse_u32_value(input, pos),
            UINT64 => Self::parse_u64(input, pos),

            FLOAT32 => Self::parse_f32(input, pos),
            FLOAT64 => Self::parse_f64(input, pos),

            _ => Err(ProtocolError::UnexpectedByte(byte).into()),
        }
    }

    #[inline]
    fn parse_str(
        input: &[u8],
        pos: &mut usize,
        len: usize,
    ) -> crate::Result<Value> {
        if *pos + len > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }

        let bytes = &input[*pos..*pos + len];
        *pos += len;

        String::from_utf8(bytes.to_vec())
            .map(Value::String)
            .map_err(|_| ProtocolError::InvalidUtf8.into())
    }

    #[inline]
    fn parse_array(
        input: &[u8],
        pos: &mut usize,
        len: usize,
    ) -> crate::Result<Value> {
        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            values.push(Self::parse_value(input, pos)?);
        }
        Ok(Value::Array(values))
    }

    #[inline]
    fn parse_map(
        input: &[u8],
        pos: &mut usize,
        len: usize,
    ) -> crate::Result<Value> {
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = match Self::parse_value(input, pos)? {
                Value::String(s) => s,
                _ => return Err(ProtocolError::InvalidMapKey.into()),
            };
            let value = Self::parse_value(input, pos)?;
            map.insert(key, value);
        }
        Ok(Value::Map(map))
    }

    #[inline]
    fn parse_u16(input: &[u8], pos: &mut usize) -> crate::Result<u16> {
        if *pos + 2 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = u16::from_be_bytes([input[*pos], input[*pos + 1]]);
        *pos += 2;
        Ok(value)
    }

    #[inline]
    fn parse_u32(input: &[u8], pos: &mut usize) -> crate::Result<u32> {
        if *pos + 4 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = u32::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
        ]);
        *pos += 4;
        Ok(value)
    }

    #[inline]
    fn parse_i8(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 1 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = input[*pos] as i8;
        *pos += 1;
        Ok(Value::Integer(value as i64))
    }

    #[inline]
    fn parse_i16(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 2 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = i16::from_be_bytes([input[*pos], input[*pos + 1]]);
        *pos += 2;
        Ok(Value::Integer(value as i64))
    }

    #[inline]
    fn parse_i32(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 4 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = i32::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
        ]);
        *pos += 4;
        Ok(Value::Integer(value as i64))
    }

    #[inline]
    fn parse_i64(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 8 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = i64::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
            input[*pos + 4],
            input[*pos + 5],
            input[*pos + 6],
            input[*pos + 7],
        ]);
        *pos += 8;
        Ok(Value::Integer(value))
    }

    #[inline]
    fn parse_f32(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 4 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = f32::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
        ]);
        *pos += 4;
        Ok(Value::Float(value as f64))
    }

    #[inline]
    fn parse_f64(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 8 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = f64::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
            input[*pos + 4],
            input[*pos + 5],
            input[*pos + 6],
            input[*pos + 7],
        ]);
        *pos += 8;
        Ok(Value::Float(value))
    }

    #[inline]
    fn parse_u8(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos >= input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = input[*pos];
        *pos += 1;
        Ok(Value::UInteger(value as u64))
    }

    #[inline]
    fn parse_u16_value(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        let value = Self::parse_u16(input, pos)?;
        Ok(Value::UInteger(value as u64))
    }

    #[inline]
    fn parse_u32_value(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        let value = Self::parse_u32(input, pos)?;
        Ok(Value::UInteger(value as u64))
    }

    #[inline]
    fn parse_u64(input: &[u8], pos: &mut usize) -> crate::Result<Value> {
        if *pos + 8 > input.len() {
            return Err(ProtocolError::InvalidLength.into());
        }
        let value = u64::from_be_bytes([
            input[*pos],
            input[*pos + 1],
            input[*pos + 2],
            input[*pos + 3],
            input[*pos + 4],
            input[*pos + 5],
            input[*pos + 6],
            input[*pos + 7],
        ]);
        *pos += 8;
        Ok(Value::UInteger(value))
    }
}
