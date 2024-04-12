pub fn get_file_browser<'a>(filetype: &'a str) -> (&'a str, &'a str) {
    match filetype {
        "netrw" => ("default", "Netrw"),
        "TelescopePrompt" => ("telescope", "Telescope"),
        "dirvish" => ("default", "Dirvish"),
        "oil" => ("default", "Oil"),
        "neo-tree" => ("default", "Neo-Tree"),
        "NvimTree" => ("default", "nvim-tree"),
        "minifiles" => ("default", "mini.files"),
        _ => ("default", &filetype),
    }
}
