use std::collections::HashMap;

pub trait Deserializable: Sized {
    fn deserialize(input: &HashMap<String, Value>) -> Result<Self, String>;
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

pub struct Json;

impl Json {
    pub fn deserialize(input: &str) -> Result<HashMap<String, Value>, String> {
        let input = input.trim();
        if !input.starts_with('{') || !input.ends_with('}') {
            return Err(format!("Invalid JSON object: {}", input));
        }

        let mut result = HashMap::new();
        let mut chars = input[1..input.len() - 1].chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_whitespace() || c == ',' {
                continue;
            }

            if c != '"' {
                return Err(format!("Expected '\"', found '{}'", c));
            }
            let key = Self::parse_string(&mut chars)?;

            if chars.next() != Some(':') {
                return Err("Expected ':' after key".to_string());
            }

            let value = Self::parse_value(&mut chars)?;
            result.insert(key, value);
        }

        Ok(result)
    }

    fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String, String> {
        let mut result = String::new();
        while let Some(&c) = chars.peek() {
            chars.next();
            match c {
                '"' => {
                    return Ok(result);
                }
                '\\' => {
                    if let Some(&next_char) = chars.peek() {
                        match next_char {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            '/' => result.push('/'),
                            'b' => result.push('\x08'),
                            'f' => result.push('\x0C'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                chars.next();
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    if let Some(&hex_char) = chars.peek() {
                                        if hex_char.is_digit(16) {
                                            hex.push(hex_char);
                                            chars.next();
                                        } else {
                                            return Err(
                                                "Invalid Unicode escape sequence".to_string()
                                            );
                                        }
                                    } else {
                                        return Err(
                                            "Incomplete Unicode escape sequence".to_string()
                                        );
                                    }
                                }
                                if let Ok(code_point) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(code_point) {
                                        result.push(ch);
                                    } else {
                                        return Err("Invalid Unicode code point".to_string());
                                    }
                                } else {
                                    return Err("Invalid Unicode escape sequence".to_string());
                                }
                            }
                            _ => return Err(format!("Invalid escape sequence: \\{}", next_char)),
                        }
                        chars.next();
                    } else {
                        return Err("Incomplete escape sequence".to_string());
                    }
                }
                _ => result.push(c),
            }
        }
        Err("Unterminated string".to_string())
    }

    fn parse_value(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        while let Some(&c) = chars.peek() {
            match c {
                '"' => {
                    chars.next();
                    return Ok(Value::String(Self::parse_string(chars)?));
                }
                '[' => {
                    chars.next();
                    return Self::parse_array(chars);
                }
                '{' => {
                    chars.next();
                    return Self::parse_nested_map(chars);
                }
                't' => {
                    chars.next();
                    return Self::parse_true(chars);
                }
                'f' => {
                    chars.next();
                    return Self::parse_false(chars);
                }
                'n' => {
                    chars.next();
                    return Self::parse_null(chars);
                }
                '-' | '0'..='9' => {
                    return Self::parse_number(chars);
                }
                _ if c.is_whitespace() => {
                    chars.next();
                    continue;
                }
                _ => return Err(format!("Unexpected character: '{}'", c)),
            }
        }
        Err("Unexpected end of input".to_string())
    }

    fn parse_array(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let mut values = Vec::new();
        let mut expecting_value = true;

        while let Some(&c) = chars.peek() {
            match c {
                ']' if !expecting_value => {
                    chars.next();
                    return Ok(Value::Array(values));
                }
                ',' if !expecting_value => {
                    chars.next();
                    expecting_value = true;
                }
                _ if c.is_whitespace() => {
                    chars.next();
                }
                _ if expecting_value => {
                    values.push(Self::parse_value(chars)?);
                    expecting_value = false;
                }
                _ => return Err(format!("Unexpected character in array: '{}'", c)),
            }
        }
        Err("Unterminated array".to_string())
    }

    fn parse_nested_map(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let mut result = HashMap::new();
        while let Some(&c) = chars.peek() {
            match c {
                '}' => {
                    chars.next();
                    return Ok(Value::Object(result));
                }
                ',' => {
                    chars.next();
                }
                '"' => {
                    chars.next();
                    let key = Self::parse_string(chars)?;

                    if chars.next() != Some(':') {
                        return Err("Expected ':' after key".to_string());
                    }

                    let value = Self::parse_value(chars)?;
                    result.insert(key, value);
                }
                _ if c.is_whitespace() => {
                    chars.next();
                }
                _ => return Err(format!("Unexpected character in map: '{}'", c)),
            }
        }
        Err("Unterminated map".to_string())
    }

    fn parse_true(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let expected = ['r', 'u', 'e'];
        for expected_char in expected {
            match chars.next() {
                Some(c) if c == expected_char => continue,
                _ => return Err("Invalid boolean 'true' value".to_string()),
            }
        }
        Ok(Value::Boolean(true))
    }

    fn parse_false(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let expected = ['a', 'l', 's', 'e'];
        for expected_char in expected {
            match chars.next() {
                Some(c) if c == expected_char => continue,
                _ => return Err("Invalid boolean 'false' value".to_string()),
            }
        }
        Ok(Value::Boolean(false))
    }

    fn parse_null(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let expected = ['u', 'l', 'l'];
        for expected_char in expected {
            match chars.next() {
                Some(c) if c == expected_char => continue,
                _ => return Err("Invalid null value".to_string()),
            }
        }
        Ok(Value::Null)
    }

    fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Value, String> {
        let mut number_str = String::new();
        let mut has_decimal = false;

        if let Some(&'-') = chars.peek() {
            number_str.push(chars.next().unwrap());
        }

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    number_str.push(chars.next().unwrap());
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    number_str.push(chars.next().unwrap());
                }
                '.' => return Err("Invalid number format: multiple decimal points".to_string()),
                _ if c.is_whitespace() || c == ',' || c == '}' || c == ']' => break,
                _ => return Err(format!("Invalid number character: '{}'", c)),
            }
        }

        number_str
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|e| format!("Failed to parse number: {}", e))
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Value::String(s) = self {
            Some(s.clone())
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        if let Value::Array(arr) = self {
            Some(arr)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        if let Value::Object(map) = self {
            Some(map)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}
