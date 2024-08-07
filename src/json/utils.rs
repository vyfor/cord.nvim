pub fn escape_json(value: &str) -> String {
    value.replace("\\", "\\\\").replace("\"", "\\\"")
}
