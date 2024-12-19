local M = {}

M.default_icon = 'text'
local mappings = {
  ['Cord.new'] = { M.default_icon, 'New file' },
  ['Cord.unknown'] = { M.default_icon, 'Unknown' },
  autohotkey = { 'ahk', 'AutoHotkey' },
  asm = { 'assembly', 'Assembly' },
  astro = { 'astro', 'Astro' },
  sh = { 'shell', 'Shell script' },
  c = { 'c', 'C' },
  clojure = { 'clojure', 'Clojure' },
  cpp = { 'cpp', 'C++' },
  cr = { 'crystal', 'Crystal' },
  cs = { 'csharp', 'C#' },
  css = { 'css', 'CSS' },
  cuda = { 'nvidia', 'CUDA' },
  d = { 'd', 'D' },
  dart = { 'dart', 'Dart' },
  dockerfile = { 'docker', 'Docker' },
  dosbatch = { 'shell', 'Batch' },
  elixir = { 'elixir', 'Elixir' },
  eelixir = { 'elixir', 'Embedded Elixir' },
  heex = { 'phoenix', 'Phoenix' },
  erlang = { 'erlang', 'Erlang' },
  fsharp = { 'fsharp', 'F#' },
  git = { 'git', 'Git' },
  gitattributes = { 'git', 'Git' },
  gitconfig = { 'git', 'Git' },
  gitignore = { 'git', 'Git' },
  gitsendemail = { 'git', 'Git' },
  go = { 'go', 'Go' },
  haskell = { 'haskell', 'Haskell' },
  html = { 'html', 'HTML' },
  java = { 'java', 'Java' },
  javascript = { 'javascript', 'JavaScript' },
  javascriptreact = { 'react', 'JSX' },
  json = { 'json', 'JSON' },
  julia = { 'julia', 'Julia' },
  tex = { 'latex', 'LaTeX' },
  texmf = { 'latex', 'LaTeX' },
  plaintex = { 'latex', 'LaTeX' },
  lisp = { 'lisp', 'Lisp' },
  lua = { 'lua', 'Lua' },
  nim = { 'nim', 'Nim' },
  nix = { 'nix', 'Nix' },
  ocaml = { 'ocaml', 'OCaml' },
  pascal = { 'pascal', 'Pascal' },
  perl = { 'perl', 'Perl' },
  php = { 'php', 'PHP' },
  ps1 = { 'powershell', 'PowerShell' },
  python = { 'python', 'Python' },
  quarto = { 'quarto', 'Quarto' },
  r = { 'r', 'R' },
  ruby = { 'ruby', 'Ruby' },
  rust = { 'rust', 'Rust' },
  scala = { 'scala', 'Scala' },
  sass = { 'scss', 'Sass' },
  scss = { 'scss', 'Sass' },
  sql = { 'sql', 'SQL' },
  svelte = { 'svelte', 'Svelte' },
  swift = { 'swift', 'Swift' },
  typescript = { 'typescript', 'TypeScript' },
  typescriptreact = { 'react', 'TSX' },
  v = { 'v', 'V' },
  vim = { 'viml', 'VimL' },
  vue = { 'vue', 'Vue' },
  yaml = { 'yaml', 'YAML' },
  zig = { 'zig', 'Zig' },
  zsh = { 'shell', 'Zsh' },
}

local special_cases = {
  groovy = function(filename)
    return filename == 'build.gradle' and { 'gradle', 'Gradle' }
      or { 'groovy', 'Groovy' }
  end,
  kotlin = function(filename)
    return filename == 'build.gradle.kts' and { 'gradle', 'Gradle' }
      or { 'kotlin', 'Kotlin' }
  end,
  markdown = function(filename)
    return filename:lower() == 'license.md' and { 'license', 'License file' }
      or { 'markdown', 'Markdown' }
  end,
  text = function(filename)
    local name = filename:lower()
    return (name == 'license' or name == 'license.txt')
        and { 'license', 'License file' }
      or { M.default_icon, 'Plain Text' }
  end,
  toml = function(filename)
    return filename == 'Cargo.toml' and { 'cargo', 'Cargo' }
      or { 'toml', 'TOML' }
  end,
  xml = function(filename)
    return filename == 'pom.xml' and { 'maven', 'Maven' } or { 'xml', 'XML' }
  end,
}

local extension_mappings = {
  gml = { 'gml', 'GameMaker Language' },
  hx = { 'haxe', 'Haxe' },
  hxml = { 'haxe', 'Haxe' },
  pcss = { 'postcss', 'PostCSS' },
  postcss = { 'postcss', 'PostCSS' },
}

M.get = function(filetype, filename)
  if special_cases[filetype] then return special_cases[filetype](filename) end
  if mappings[filetype] then return mappings[filetype] end

  local ext = filename:match '%.([^%.]+)$'
  if ext then return extension_mappings[ext:lower()] end
end

return M
