#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Language,
    FileBrowser,
    PluginManager,
    LspManager,
    Vcs,
}

impl AssetType {
    #[inline(always)]
    pub fn from(value: &str) -> Option<AssetType> {
        match value {
            "language" => Some(AssetType::Language),
            "file_browser" => Some(AssetType::FileBrowser),
            "plugin_manager" => Some(AssetType::PluginManager),
            "lsp_manager" => Some(AssetType::LspManager),
            "vcs" => Some(AssetType::Vcs),
            _ => None,
        }
    }
}

impl TryFrom<u64> for AssetType {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AssetType::Language),
            1 => Ok(AssetType::FileBrowser),
            2 => Ok(AssetType::PluginManager),
            3 => Ok(AssetType::LspManager),
            4 => Ok(AssetType::Vcs),
            _ => Err(()),
        }
    }
}
