#![allow(clippy::too_many_arguments)]

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

#[macro_export]
macro_rules! get_field_or_none {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.get($field).and_then($expr)
    };
}

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

#[macro_export]
macro_rules! remove_field_or_none {
    ($map:expr, $field:expr, $expr:expr) => {
        $map.remove($field).and_then($expr)
    };
}
