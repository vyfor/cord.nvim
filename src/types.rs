pub enum AssetType {
    Language,
    FileBrowser,
    PluginManager,
    LSP,
}

impl AssetType {
    #[inline(always)]
    pub fn from(value: i32) -> Option<AssetType> {
        match value {
            0 => Some(AssetType::Language),
            1 => Some(AssetType::FileBrowser),
            2 => Some(AssetType::PluginManager),
            3 => Some(AssetType::LSP),
            _ => None,
        }
    }
}
