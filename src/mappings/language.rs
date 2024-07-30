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
        "eelixir" => ("elixir", "Embedded Elixir"),
        "heex" => ("phoenix", "Phoenix"),
        "erlang" => ("erlang", "Erlang"),
        "fsharp" => ("fsharp", "F#"),
        "git" | "gitattributes" | "gitconfig" | "gitignore"
        | "gitsendemail" => ("git", "Git"),
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
        "julia" => ("julia", "Julia"),
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
        "markdown" => {
            if filename.to_lowercase() == "license.md" {
                ("license", "License file")
            } else {
                ("markdown", "Markdown")
            }
        }
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
            let filename = filename.to_lowercase();
            if filename == "license" || filename == "license.txt" {
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
        "xml" => {
            if filename == "pom.xml" {
                ("maven", "Maven")
            } else {
                ("xml", "XML")
            }
        }
        "yaml" => ("yaml", "YAML"),
        "zig" => ("zig", "Zig"),
        "zsh" => ("shell", "Zsh"),
        _ => match filename.to_lowercase().rsplit_once('.') {
            Some((_, extension)) => match extension {
                "gml" => ("gml", "GameMaker Language"),
                "pcss" | "postcss" => ("postcss", "PostCSS"),
                _ => return None,
            },
            None => return None,
        },
    };

    Some(language)
}
