pub fn get_plugin_manager<'a>(filetype: &'a str) -> (&'a str, &'a str) {
    match filetype {
        "lazy" => ("default", "Lazy"),
        _ => ("default", &filetype),
    }
}
