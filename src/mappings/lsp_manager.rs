pub fn get(filetype: &str) -> Option<(&str, &str)> {
    let lsp_manager = match filetype {
        "lspinfo" => ("default", "lspconfig"),
        "mason" => ("default", "Mason"),
        _ => return None,
    };

    Some(lsp_manager)
}
