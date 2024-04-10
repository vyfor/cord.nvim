use std::collections::HashMap;

pub static mut PLUGIN_MANAGERS: Option<HashMap<&str, (&str, &str)>> = None; // <filetype, (icon, name)>

pub fn init() {
    unsafe {
        PLUGIN_MANAGERS =
            Some([("lazy", ("default", "Lazy"))].iter().copied().collect())
    }
}
