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
