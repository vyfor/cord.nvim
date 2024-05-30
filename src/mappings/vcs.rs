pub fn get(filetype: &str) -> Option<(&str, &str)> {
    let vcs = match filetype {
        "gitcommit" | "gitrebase" => ("default", "Git"),
        "fugitive" | "fugitiveblame" => ("default", "Fugitive"),
        "magit" => ("default", "Magit"),
        "git.nvim" => ("default", "Git.nvim"),
        "lazygit" => ("default", "Lazygit"),
        _ => {
            if filetype.starts_with("Neogit") {
                ("default", "Neogit")
            } else {
                return None;
            }
        }
    };

    Some(vcs)
}
