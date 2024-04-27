pub fn get<'a>(filetype: &'a str) -> Option<(&'a str, &'a str)> {
    let lsp_manager = match filetype {
        "lspinfo" => ("default", "LSP Config"),
        "mason" => ("default", "Mason"),
        _ => return None,
    };

    Some(lsp_manager)
}
