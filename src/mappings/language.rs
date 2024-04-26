pub fn get<'a>(
    filetype: &'a str,
    filename: &str,
) -> Option<(&'a str, &'a str)> {
    let language = match filetype {
        "Cord.new" => ("text", "New file"),
        "Cord.unknown" => ("text", filetype),
        "autohotkey" => ("ahk", "AutoHotkey"),
        "asm" => ("assembly", "Assembly"),
        "sh" => ("shell", "Shell script"),
        "c" => ("c", "C "),
        "clojure" => ("clojure", "Clojure"),
        "cpp" => ("cpp", "C++"),
        "cr" => ("crystal", "Crystal"),
        "cs" => ("csharp", "C#"),
        "css" => ("css", "CSS"),
        "d" => ("d", "D "),
        "dart" => ("dart", "Dart"),
        "dockerfile" => ("docker", "Docker"),
        "dosbatch" => ("shell", "Batch"),
        "elixir" => ("elixir", "Elixir"),
        "erlang" => ("erlang", "Erlang"),
        "fsharp" => ("fsharp", "F#"),
        "git" | "gitignore" => ("git", "Git"),
        "go" => ("go", "Go"),
        "groovy" => {
            if filename == "build.gradle" {
                ("gradle", "Gradle")
            } else {
                ("groovy", "Groovy")
            }
        }
        "haskell" => ("haskell", "Haskell"),
        "html" => ("html", "HTML"),
        "java" => ("java", "Java"),
        "javascript" => ("javascript", "JavaScript"),
        "javascriptreact" => ("react", "JSX"),
        "json" => ("json", "JSON"),
        "kotlin" => {
            if filename == "build.gradle.kts" {
                ("gradle", "Gradle")
            } else {
                ("kotlin", "Kotlin")
            }
        }
        "tex" | "texmf" | "plaintex" => ("latex", "LaTeX"),
        "lisp" => ("lisp", "Lisp"),
        "lua" => ("lua", "Lua"),
        "markdown" => ("markdown", "Markdown"),
        "nim" => ("nim", "Nim"),
        "nix" => ("nix", "Nix"),
        "ocaml" => ("ocaml", "OCaml"),
        "pascal" => ("pascal", "Pascal"),
        "perl" => ("perl", "Perl"),
        "php" => ("php", "PHP"),
        "ps1" => ("powershell", "PowerShell"),
        "python" => ("python", "Python"),
        "r" => ("r", "R "),
        "ruby" => ("ruby", "Ruby"),
        "rust" => ("rust", "Rust"),
        "scala" => ("scala", "Scala"),
        "sass" | "scss" => ("scss", "Sass"),
        "sql" => ("sql", "SQL"),
        "svelte" => ("svelte", "Svelte"),
        "swift" => ("swift", "Swift"),
        "text" => {
            if filename.to_lowercase() == "license" {
                ("license", "License file")
            } else {
                ("text", "Plain Text")
            }
        }
        "toml" => {
            if filename == "Cargo.toml" {
                ("cargo", "Cargo")
            } else {
                ("toml", "TOML")
            }
        }
        "typescript" => ("typescript", "TypeScript"),
        "typescriptreact" => ("react", "TSX"),
        "v" => ("v", "V "),
        "vim" => ("vim", "VimL"),
        "vue" => ("vue", "Vue"),
        "xml" => ("xml", "XML"),
        "yaml" => ("yaml", "YAML"),
        "zig" => ("zig", "Zig"),
        _ => match filename.to_lowercase().rsplit_once('.') {
            Some((_, extension)) => match extension {
                "gml" => ("gml", "Game Maker Language"),
                "pcss" | "postcss" => ("postcss", "PostCSS"),
                _ => return None,
            },
            None => return None,
        },
    };

    Some(language)
}
