#![allow(dead_code)]

use super::{value::ValueRef, Json};

pub type SerializeFn<'a> = fn(&'a str, ValueRef<'a>, &mut SerializeState) -> crate::Result<()>;

/// Trait for serializing Rust types into JSON data.
///
/// This trait defines a method for converting a Rust data structure into
/// a JSON representation. It requires implementing the `serialize` method
/// that uses a `SerializeFn` and `SerializeState` to produce the JSON output.
pub trait Serialize {
    /// Serializes a Rust type into JSON data.
    ///
    /// # Arguments
    ///
    /// * `f` - A function for serializing key-value pairs.
    /// * `state` - A mutable reference to the serialization state.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure of the serialization process.
    fn serialize<'a>(&'a self, f: SerializeFn<'a>, state: &mut SerializeState)
        -> crate::Result<()>;
}

pub trait SerializeObj: Serialize + std::fmt::Debug {}
impl<T: Serialize + std::fmt::Debug> SerializeObj for T {}

pub struct SerializeState {
    buf: String,
    stack: Vec<usize>,
}

impl SerializeState {
    fn new() -> Self {
        Self {
            buf: String::with_capacity(512),
            stack: Vec::with_capacity(8),
        }
    }

    fn push_scope(&mut self) {
        self.stack.push(self.buf.len());
    }

    fn needs_comma(&self) -> bool {
        if let Some(&last_pos) = self.stack.last() {
            self.buf.len() > last_pos + 1
        } else {
            false
        }
    }
}

impl std::fmt::Debug for ValueRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueRef::String(s) => write!(f, "String({:?})", s),
            ValueRef::Number(n) => write!(f, "Number({:?})", n),
            ValueRef::Boolean(b) => write!(f, "Boolean({:?})", b),
            ValueRef::Array(arr) => f.debug_tuple("Array").field(arr).finish(),
            ValueRef::Object(_) => write!(f, "Object(function)"),
            ValueRef::Null => write!(f, "Null"),
        }
    }
}

impl Json {
    pub fn serialize(value: &dyn Serialize) -> crate::Result<String> {
        let mut state = SerializeState::new();
        state.buf.push('{');
        state.push_scope();

        fn write_kv(key: &str, value: &ValueRef, state: &mut SerializeState) -> crate::Result<()> {
            if state.needs_comma() {
                state.buf.push(',');
            }
            state.buf.push('"');
            escape_str_to_buf(key, &mut state.buf);
            state.buf.push_str("\":");
            write_value(value, state)
        }

        fn serialize_adapter(
            key: &str,
            value: ValueRef,
            state: &mut SerializeState,
        ) -> crate::Result<()> {
            write_kv(key, &value, state)
        }

        value.serialize(serialize_adapter, &mut state)?;
        state.buf.push('}');
        Ok(state.buf)
    }
}

fn write_value(value: &ValueRef, state: &mut SerializeState) -> crate::Result<()> {
    match value {
        ValueRef::String(s) => {
            state.buf.push('"');
            escape_str_to_buf(s, &mut state.buf);
            state.buf.push('"');
            Ok(())
        }
        ValueRef::Number(n) => {
            use std::fmt::Write;
            write!(state.buf, "{}", n).map_err(|e| e.to_string())?;
            Ok(())
        }
        ValueRef::Boolean(b) => {
            state.buf.push_str(if *b { "true" } else { "false" });
            Ok(())
        }
        ValueRef::Array(arr) => {
            state.buf.push('[');
            state.push_scope();

            for item in arr {
                if state.needs_comma() {
                    state.buf.push(',');
                }
                write_value(item, state)?;
            }

            state.stack.pop();
            state.buf.push(']');
            Ok(())
        }
        ValueRef::Object(obj) => {
            state.buf.push('{');
            state.push_scope();

            obj.serialize(
                |key, value, state| {
                    if state.needs_comma() {
                        state.buf.push(',');
                    }
                    state.buf.push('"');
                    escape_str_to_buf(key, &mut state.buf);
                    state.buf.push_str("\":");
                    write_value(&value, state)
                },
                state,
            )?;

            state.stack.pop();
            state.buf.push('}');
            Ok(())
        }
        ValueRef::Null => {
            state.buf.push_str("null");
            Ok(())
        }
    }
}

#[inline]
fn escape_str_to_buf(s: &str, buf: &mut String) {
    for c in s.chars() {
        match c {
            '"' => buf.push_str("\\\""),
            '\\' => buf.push_str("\\\\"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            '\t' => buf.push_str("\\t"),
            c if c.is_control() => {
                buf.push_str("\\u");
                for byte in format!("{:04x}", c as u32).bytes() {
                    buf.push(byte as char);
                }
            }
            c => buf.push(c),
        }
    }
}
