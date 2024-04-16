pub mod file_browser;
pub mod language;
pub mod plugin_manager;

pub fn get_by_filetype<'a>(
    filetype: &'a str,
    filename: &'a str,
) -> Filetype<'a> {
    language::get(filetype, filename)
        .map(|language| Filetype::Language(language.0, language.1))
        .unwrap_or_else(|| {
            file_browser::get(filetype)
                .map(|file_browser| {
                    Filetype::FileBrowser(file_browser.0, file_browser.1)
                })
                .unwrap_or_else(|| {
                    plugin_manager::get(filetype)
                        .map(|plugin_manager| {
                            Filetype::PluginManager(
                                plugin_manager.0,
                                plugin_manager.1,
                            )
                        })
                        .unwrap_or_else(|| {
                            Filetype::Language("default", filetype)
                        })
                })
        })
}

pub enum Filetype<'a> {
    Language(&'a str, &'a str),
    FileBrowser(&'a str, &'a str),
    PluginManager(&'a str, &'a str),
}
