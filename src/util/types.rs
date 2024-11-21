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
