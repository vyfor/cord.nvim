pub fn get_language<'a>(
    filetype: &'a str,
    filename: &str,
) -> (&'a str, &'a str) {
    match filetype {
        "asm" => ("assembly", "Assembly"),
        "bash" => ("shell", "Bash"),
        "c" => ("c", "C"),
        "cpp" => ("cpp", "C++"),
        "csharp" => ("csharp", "C#"),
        "css" => ("css", "CSS"),
        "dart" => ("dart", "Dart"),
        "dosbatch" => ("shell", "Batch"),
        "go" => ("go", "Go"),
        "haskell" => ("haskell", "Haskell"),
        "html" => ("html", "HTML"),
        "java" => ("java", "Java"),
        "javascript" => ("javascript", "JavaScript"),
        "javascriptreact" => ("react", "JSX"),
        "json" => ("json", "JSON"),
        "kotlin" => ("kotlin", "Kotlin"),
        "lua" => ("lua", "Lua"),
        "markdown" => ("markdown", "Markdown"),
        "perl" => ("perl", "Perl"),
        "php" => ("php", "PHP"),
        "ps1" => ("powershell", "PowerShell"),
        "python" => ("python", "Python"),
        "ruby" => ("ruby", "Ruby"),
        "rust" => ("rust", "Rust"),
        "scala" => ("scala", "Scala"),
        "sql" => ("sql", "SQL"),
        "swift" => ("swift", "Swift"),
        "txt" => ("text", "Plain Text"),
        "toml" => {
            if filename == "Cargo.toml" {
                ("cargo", "Cargo")
            } else {
                ("toml", "TOML")
            }
        }
        "typescript" => ("typescript", "TypeScript"),
        "typescriptreact" => ("react", "TSX"),
        "vim" => ("vim", "VimL"),
        "xml" => ("xml", "XML"),
        "yaml" => ("yaml", "YAML"),
        _ => ("text", &filetype),
    }
}
