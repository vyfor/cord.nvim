pub mod file_browser;
pub mod language;
pub mod lsp_manager;
pub mod plugin_manager;
pub mod vcs;

pub fn get_by_filetype(filetype: &str, filename: &str) -> Filetype {
    if let Some(language) = language::get(filetype, filename) {
        return Filetype::Language(language.0.to_string(), language.1.to_string());
    }
    if let Some(file_browser) = file_browser::get(filetype) {
        return Filetype::FileBrowser(file_browser.0.to_string(), file_browser.1.to_string());
    }
    if let Some(plugin_manager) = plugin_manager::get(filetype) {
        return Filetype::PluginManager(plugin_manager.0.to_string(), plugin_manager.1.to_string());
    }
    if let Some(lsp_manager) = lsp_manager::get(filetype) {
        return Filetype::Lsp(lsp_manager.0.to_string(), lsp_manager.1.to_string());
    }
    if let Some(vcs) = vcs::get(filetype) {
        return Filetype::Vcs(vcs.0.to_string(), vcs.1.to_string());
    }
    Filetype::Language("text".to_string(), filetype.to_string())
}

pub fn get_by_filetype_or_none(filetype: &str, filename: &str) -> Option<Filetype> {
    let mut ft = None;
    if let Some(language) = language::get(filetype, filename) {
        ft = Some(Filetype::Language(
            language.0.to_string(),
            language.1.to_string(),
        ));
    }
    if let Some(file_browser) = file_browser::get(filetype) {
        ft = Some(Filetype::FileBrowser(
            file_browser.0.to_string(),
            file_browser.1.to_string(),
        ));
    }
    if let Some(plugin_manager) = plugin_manager::get(filetype) {
        ft = Some(Filetype::PluginManager(
            plugin_manager.0.to_string(),
            plugin_manager.1.to_string(),
        ));
    }
    if let Some(lsp_manager) = lsp_manager::get(filetype) {
        ft = Some(Filetype::Lsp(
            lsp_manager.0.to_string(),
            lsp_manager.1.to_string(),
        ));
    }
    if let Some(vcs) = vcs::get(filetype) {
        ft = Some(Filetype::Vcs(vcs.0.to_string(), vcs.1.to_string()));
    }

    ft
}

#[derive(Debug, Clone)]
pub enum Filetype {
    Language(String, String),
    FileBrowser(String, String),
    PluginManager(String, String),
    Lsp(String, String),
    Vcs(String, String),
}
