pub fn get(filetype: &str) -> Option<(&str, &str)> {
    let file_browser = match filetype {
        "netrw" => ("default", "Netrw"),
        "TelescopePrompt" => ("telescope", "Telescope"),
        "dirvish" => ("default", "Dirvish"),
        "oil" => ("default", "Oil"),
        "neo-tree" => ("default", "Neo-Tree"),
        "NvimTree" => ("default", "nvim-tree"),
        "minifiles" => ("default", "mini.files"),
        _ => return None,
    };

    Some(file_browser)
}
