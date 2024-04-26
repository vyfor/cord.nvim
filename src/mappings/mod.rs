pub mod file_browser;
pub mod language;
pub mod plugin_manager;

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
    Filetype::Language("text", filetype)
}

pub enum Filetype<'a> {
    Language(&'a str, &'a str),
    FileBrowser(&'a str, &'a str),
    PluginManager(&'a str, &'a str),
}
