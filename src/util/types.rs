#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Language,
    FileBrowser,
    PluginManager,
    LspManager,
    Vcs,
}

impl TryFrom<&str> for AssetType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "language" => Ok(AssetType::Language),
            "file_browser" => Ok(AssetType::FileBrowser),
            "plugin_manager" => Ok(AssetType::PluginManager),
            "lsp_manager" => Ok(AssetType::LspManager),
            "vcs" => Ok(AssetType::Vcs),
            _ => Err(()),
        }
    }
}
