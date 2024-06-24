pub mod file_browser;
pub mod language;
pub mod lsp_manager;
pub mod plugin_manager;
pub mod vcs;

pub fn get_by_filetype<'a>(filetype: &'a str, filename: &str) -> Filetype<'a> {
    if let Some(language) = language::get(filetype, filename) {
        return Filetype::Language(language.0, language.1);
    }
    if let Some(file_browser) = file_browser::get(filetype) {
        return Filetype::FileBrowser(file_browser.0, file_browser.1);
    }
    if let Some(plugin_manager) = plugin_manager::get(filetype) {
        return Filetype::PluginManager(plugin_manager.0, plugin_manager.1);
    }
    if let Some(lsp_manager) = lsp_manager::get(filetype) {
        return Filetype::Lsp(lsp_manager.0, lsp_manager.1);
    }
    if let Some(vcs) = vcs::get(filetype) {
        return Filetype::Vcs(vcs.0, vcs.1);
    }
    Filetype::Language("text", filetype)
}

pub fn get_by_filetype_or_none<'a>(
    filetype: &'a str,
    filename: &str,
) -> Option<Filetype<'a>> {
    let mut ft = None;
    if let Some(language) = language::get(filetype, filename) {
        ft = Some(Filetype::Language(language.0, language.1));
    }
    if let Some(file_browser) = file_browser::get(filetype) {
        ft = Some(Filetype::FileBrowser(file_browser.0, file_browser.1));
    }
    if let Some(plugin_manager) = plugin_manager::get(filetype) {
        ft = Some(Filetype::PluginManager(plugin_manager.0, plugin_manager.1));
    }
    if let Some(lsp_manager) = lsp_manager::get(filetype) {
        ft = Some(Filetype::Lsp(lsp_manager.0, lsp_manager.1));
    }
    if let Some(vcs) = vcs::get(filetype) {
        ft = Some(Filetype::Vcs(vcs.0, vcs.1));
    }

    ft
}

pub enum Filetype<'a> {
    Language(&'a str, &'a str),
    FileBrowser(&'a str, &'a str),
    PluginManager(&'a str, &'a str),
    Lsp(&'a str, &'a str),
    Vcs(&'a str, &'a str),
}
