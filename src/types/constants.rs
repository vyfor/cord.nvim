use std::{collections::HashMap, sync::LazyLock};

pub static CLIENT_IDS: LazyLock<HashMap<&str, u64>> = LazyLock::new(|| {
    HashMap::from([
        ("vim", 1219918645770059796),
        ("neovim", 1219918880005165137),
        ("lunarvim", 1220295374087000104),
        ("nvchad", 1220296082861326378),
        ("astronvim", 1230866983977746532),
    ])
});
