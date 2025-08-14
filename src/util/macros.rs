/// Retrieves a field from a map, applies a transformation, and returns an error if the field is missing or invalid.
///
/// This macro is useful for extracting and validating fields from a map-like structure.
#[macro_export]
macro_rules! get_field {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.get($field).and_then($expr).ok_or(concat!(
            "Missing or invalid '",
            $field,
            "' field"
        ))?
    };
}

/// Retrieves a field from a map and applies a transformation, returning `None` if the field is missing or invalid.
///
/// This macro is useful for optional field extraction and transformation.
#[macro_export]
macro_rules! get_field_or_none {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.get($field).and_then($expr)
    };
}

/// Removes a field from a map, applies a transformation, and returns an error if the field is missing or invalid.
///
/// This macro is useful for extracting, validating, and removing fields from a map-like structure.
#[macro_export]
macro_rules! remove_field {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.remove($field).and_then($expr).ok_or(concat!(
            "Missing or invalid '",
            $field,
            "' field"
        ))?
    };
}

/// Removes a field from a map and applies a transformation, returning `None` if the field is missing or invalid.
///
/// This macro is useful for optional field extraction, transformation, and removal.
#[macro_export]
macro_rules! remove_field_or_none {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.remove($field).and_then($expr)
    };
}

/// Prints a message to stdout without panicking.
#[macro_export]
macro_rules! echo {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = write!(handle, $($arg)*);
    }};
}

/// Prints a message with a newline appended to stdout without panicking.
#[macro_export]
macro_rules! echoln {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = writeln!(handle, $($arg)*);
    }};
}

/// Prints a message to stderr without panicking.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        let _ = writeln!(handle, $($arg)*);
    }};
}
