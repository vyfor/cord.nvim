pub fn get_file_browser<'a>(filetype: &'a str) -> (&'a str, &'a str) {
    match filetype {
        "netrw" => ("default", "Netrw"),
        "TelescopePrompt" => ("telescope", "Telescope"),
        "dirvish" => ("default", "Dirvish"),
        _ => ("default", &filetype),
    }
}
