const DEFAULT: &str = "default_file_browser";

pub fn get<'a>(filetype: &'a str) -> Option<(&'a str, &'a str)> {
    let file_browser = match filetype {
        "netrw" => (DEFAULT, "Netrw"),
        "TelescopePrompt" => ("telescope", "Telescope"),
        "dirvish" => (DEFAULT, "Dirvish"),
        "oil" => (DEFAULT, "Oil"),
        "neo-tree" => (DEFAULT, "Neo-Tree"),
        "NvimTree" => (DEFAULT, "nvim-tree"),
        "minifiles" => (DEFAULT, "mini.files"),
        _ => return None,
    };

    Some(file_browser)
}
