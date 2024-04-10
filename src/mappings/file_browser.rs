use std::collections::HashMap;

pub static mut FILE_BROWSERS: Option<HashMap<&str, (&str, &str)>> = None; // <filetype, (icon, name)>

pub fn init() {
    unsafe {
        FILE_BROWSERS = Some(
            [
                ("netrw", ("default", "Netrw")),
                ("TelescopePrompt", ("telescope", "Telescope")),
                ("dirvish", ("default", "Dirvish")),
            ]
            .iter()
            .copied()
            .collect(),
        );
    }
}
