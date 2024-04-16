pub fn get<'a>(filetype: &'a str) -> Option<(&'a str, &'a str)> {
    let plugin_manager = match filetype {
        "lazy" => ("default", "Lazy"),
        "pckr" => ("default", "Pckr"),
        "packer" => ("default", "Packer"),
        _ => return None,
    };

    Some(plugin_manager)
}
