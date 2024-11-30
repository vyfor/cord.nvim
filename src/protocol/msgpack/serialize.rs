#![allow(dead_code)]

use super::{
    value::ValueRef, MsgPack, ARRAY16, ARRAY32, FALSE, FIXARRAY_VALUE, FIXMAP_SIZE_MASK,
    FIXMAP_VALUE, FIXSTR_VALUE, FLOAT64, INT64, MAP16, MAP32, NIL, STR16, STR32, TRUE, UINT16,
    UINT32, UINT64, UINT8,
};
use crate::protocol::error::ProtocolError;

pub type SerializeFn<'a> = fn(&'a str, ValueRef<'a>, &mut SerializeState) -> crate::Result<()>;

pub trait Serialize {
    fn serialize<'a>(&'a self, f: SerializeFn<'a>, state: &mut SerializeState)
        -> crate::Result<()>;
}

pub trait SerializeObj: Serialize + std::fmt::Debug {}
impl<T: Serialize + std::fmt::Debug> SerializeObj for T {}

pub struct SerializeState {
    buf: Vec<u8>,
    stack: Vec<usize>,
}

impl SerializeState {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(512),
            stack: Vec::with_capacity(8),
        }
    }

    fn push_scope(&mut self) {
        self.stack.push(self.buf.len());
    }

    fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    fn write_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn write_i64(&mut self, v: i64) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn write_f64(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_be_bytes());
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }
}

impl std::fmt::Debug for ValueRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueRef::String(s) => write!(f, "String({:?})", s),
            ValueRef::Float(n) => write!(f, "Number({:?})", n),
            ValueRef::Integer(i) => write!(f, "Integer({:?})", i),
            ValueRef::UInteger(u) => write!(f, "UInteger({:?})", u),
            ValueRef::Boolean(b) => write!(f, "Boolean({:?})", b),
            ValueRef::Array(arr) => f.debug_tuple("Array").field(arr).finish(),
            ValueRef::Object(_) => write!(f, "Object(function)"),
            ValueRef::Map(map) => f.debug_tuple("Map").field(map).finish(),
            ValueRef::Nil => write!(f, "Nil"),
        }
    }
}

impl MsgPack {
    pub fn serialize(value: &dyn Serialize) -> crate::Result<Vec<u8>> {
        let mut state = SerializeState::new();
        state.write_u8(0x81);

        fn serialize_adapter(
            key: &str,
            value: ValueRef,
            state: &mut SerializeState,
        ) -> crate::Result<()> {
            MsgPack::write_kv(key, &value, state)
        }

        value.serialize(serialize_adapter, &mut state)?;
        Ok(state.buf)
    }

    fn write_str(s: &str, state: &mut SerializeState) -> crate::Result<()> {
        let bytes = s.as_bytes();
        let len = bytes.len();
        match len {
            0..=31 => {
                state.write_u8(FIXSTR_VALUE | (len as u8));
            }
            32..=65535 => {
                state.write_u8(STR16);
                state.write_u16(len as u16);
            }
            _ => {
                if len > u32::MAX as usize {
                    return Err(ProtocolError::InvalidLength.into());
                }
                state.write_u8(STR32);
                state.write_u32(len as u32);
            }
        }
        state.write_bytes(bytes);
        Ok(())
    }

    fn write_value(value: &ValueRef, state: &mut SerializeState) -> crate::Result<()> {
        match value {
            ValueRef::Nil => {
                state.write_u8(NIL);
                Ok(())
            }
            ValueRef::String(s) => Self::write_str(s, state),
            ValueRef::Float(n) => {
                state.write_u8(FLOAT64);
                state.write_f64(*n);
                Ok(())
            }
            ValueRef::Integer(n) => {
                if *n >= -(1 << 5) && *n < (1 << 7) {
                    state.write_u8(*n as u8);
                } else {
                    state.write_u8(INT64);
                    state.write_i64(*n);
                }
                Ok(())
            }
            ValueRef::UInteger(n) => {
                if *n < (1 << 8) {
                    state.write_u8(UINT8);
                    state.write_u8(*n as u8);
                } else if *n < (1 << 16) {
                    state.write_u8(UINT16);
                    state.write_u16(*n as u16);
                } else if *n < (1 << 32) {
                    state.write_u8(UINT32);
                    state.write_u32(*n as u32);
                } else {
                    state.write_u8(UINT64);
                    state.write_u64(*n);
                }
                Ok(())
            }
            ValueRef::Boolean(b) => {
                state.write_u8(if *b { TRUE } else { FALSE });
                Ok(())
            }
            ValueRef::Array(arr) => {
                let len = arr.len();
                match len {
                    0..=15 => {
                        state.write_u8(FIXARRAY_VALUE | (len as u8));
                    }
                    16..=65535 => {
                        state.write_u8(ARRAY16);
                        state.write_u16(len as u16);
                    }
                    _ => {
                        if len > u32::MAX as usize {
                            return Err(ProtocolError::InvalidLength.into());
                        }
                        state.write_u8(ARRAY32);
                        state.write_u32(len as u32);
                    }
                }
                for item in arr {
                    Self::write_value(item, state)?;
                }
                Ok(())
            }
            ValueRef::Object(obj) => {
                state.push_scope();
                obj.serialize(|k, v, s| Self::write_kv(k, &v, s), state)?;
                Ok(())
            }
            ValueRef::Map(map) => {
                let len = map.len();
                match len {
                    0..=15 => {
                        state.write_u8(FIXMAP_VALUE | (len as u8));
                    }
                    16..=65535 => {
                        state.write_u8(MAP16);
                        state.write_u16(len as u16);
                    }
                    _ => {
                        if len > u32::MAX as usize {
                            return Err(ProtocolError::InvalidLength.into());
                        }
                        state.write_u8(MAP32);
                        state.write_u32(len as u32);
                    }
                }
                for (k, v) in map {
                    Self::write_str(k, state)?;
                    Self::write_value(v, state)?;
                }
                Ok(())
            }
        }
    }

    fn write_kv(key: &str, value: &ValueRef, state: &mut SerializeState) -> crate::Result<()> {
        Self::write_str(key, state)?;
        Self::write_value(value, state)
    }
}
