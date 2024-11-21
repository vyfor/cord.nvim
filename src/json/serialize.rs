#![allow(dead_code)]

use super::Json;

#[repr(u8)]
pub enum SValue<'a> {
    Null = 0,
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Array(Vec<SValue<'a>>),
    Object(&'a dyn Serialize),
}

pub type SerializeFn<'a> = fn(&'a str, SValue<'a>, &mut SerializeState) -> Result<(), String>;

pub trait Serialize {
    fn serialize<'a>(
        &'a self,
        f: SerializeFn<'a>,
        state: &mut SerializeState,
    ) -> Result<(), String>;
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

impl<'a> std::fmt::Debug for SValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SValue::String(s) => write!(f, "String({:?})", s),
            SValue::Number(n) => write!(f, "Number({:?})", n),
            SValue::Boolean(b) => write!(f, "Boolean({:?})", b),
            SValue::Array(arr) => f.debug_tuple("Array").field(arr).finish(),
            SValue::Object(_) => write!(f, "Object(function)"),
            SValue::Null => write!(f, "Null"),
        }
    }
}

impl Json {
    pub fn serialize(value: &dyn Serialize) -> Result<String, String> {
        let mut state = SerializeState::new();
        state.buf.push('{');
        state.push_scope();

        fn write_kv(key: &str, value: &SValue, state: &mut SerializeState) -> Result<(), String> {
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
            value: SValue,
            state: &mut SerializeState,
        ) -> Result<(), String> {
            write_kv(key, &value, state)
        }

        value.serialize(serialize_adapter, &mut state)?;
        state.buf.push('}');
        Ok(state.buf)
    }
}

fn write_value(value: &SValue, state: &mut SerializeState) -> Result<(), String> {
    match value {
        SValue::String(s) => {
            state.buf.push('"');
            escape_str_to_buf(s, &mut state.buf);
            state.buf.push('"');
            Ok(())
        }
        SValue::Number(n) => {
            use std::fmt::Write;
            write!(state.buf, "{}", n).map_err(|e| e.to_string())
        }
        SValue::Boolean(b) => {
            state.buf.push_str(if *b { "true" } else { "false" });
            Ok(())
        }
        SValue::Array(arr) => {
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
        SValue::Object(obj) => {
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
        SValue::Null => {
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
