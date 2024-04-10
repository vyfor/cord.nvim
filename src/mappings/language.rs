use std::collections::HashMap;

pub static mut LANGUAGES: Option<HashMap<&str, (&str, &str)>> = None; // <filetype, (icon, name)>

pub fn init() {
    unsafe {
        LANGUAGES = Some(
            [
                ("asm", ("assembly", "Assembly")),
                ("bash", ("bash", "Bash")),
                ("c", ("c", "C")),
                ("cpp", ("cpp", "C++")),
                ("csharp", ("csharp", "C#")),
                ("css", ("css", "CSS")),
                ("dart", ("dart", "Dart")),
                ("go", ("go", "Go")),
                ("haskell", ("haskell", "Haskell")),
                ("html", ("html", "HTML")),
                ("java", ("java", "Java")),
                ("javascript", ("javascript", "JavaScript")),
                ("javascriptreact", ("react", "JSX")),
                ("json", ("json", "JSON")),
                ("kotlin", ("kotlin", "Kotlin")),
                ("lua", ("lua", "Lua")),
                ("markdown", ("markdown", "Markdown")),
                ("perl", ("perl", "Perl")),
                ("php", ("php", "PHP")),
                ("ps1", ("powershell", "PowerShell")),
                ("python", ("python", "Python")),
                ("ruby", ("ruby", "Ruby")),
                ("rust", ("rust", "Rust")),
                ("scala", ("scala", "Scala")),
                ("sql", ("sql", "SQL")),
                ("swift", ("swift", "Swift")),
                ("txt", ("text", "Plain Text")),
                ("toml", ("toml", "TOML")),
                ("typescript", ("typescript", "TypeScript")),
                ("typescriptreact", ("react", "TSX")),
                ("vim", ("vim", "VimL")),
                ("xml", ("xml", "XML")),
                ("yaml", ("yaml", "YAML")),
            ]
            .iter()
            .copied()
            .collect(),
        )
    }
}
