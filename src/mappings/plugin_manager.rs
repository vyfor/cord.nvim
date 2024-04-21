const DEFAULT: &str = "default_plugin_manager";

pub fn get<'a>(filetype: &'a str) -> Option<(&'a str, &'a str)> {
    let plugin_manager = match filetype {
        "lazy" => (DEFAULT, "Lazy"),
        "pckr" => (DEFAULT, "Pckr"),
        "packer" => (DEFAULT, "Packer"),
        _ => return None,
    };

    Some(plugin_manager)
}
