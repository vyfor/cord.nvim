local config = require('cord.plugin.config').opts

local M = {}

M.default_icons = {
  language = 'text',
  file_browser = 'folder',
  plugin_manager = 'plugin',
  lsp = 'lsp',
  docs = 'book',
  vcs = 'git',
  notes = 'notes',
  dashboard = 'dashboard',
}

M.mappings = {
  -- Languages
  autohotkey = { 'language', 'ahk', 'AutoHotkey' },
  asm = { 'language', 'assembly', 'Assembly' },
  astro = { 'language', 'astro', 'Astro' },
  awk = { 'language', 'awk', 'Awk' },
  c = { 'language', 'c', 'C ' },
  clojure = { 'language', 'clojure', 'Clojure' },
  cpp = { 'language', 'cpp', 'C++' },
  cr = { 'language', 'crystal', 'Crystal' },
  cs = { 'language', 'csharp', 'C#' },
  css = { 'language', 'css', 'CSS' },
  cuda = { 'language', 'nvidia', 'CUDA' },
  d = { 'language', 'd', 'D ' },
  dart = { 'language', 'dart', 'Dart' },
  dockerfile = { 'language', 'docker', 'Docker' },
  dosbatch = { 'language', 'shell', 'Batch' },
  elixir = { 'language', 'elixir', 'Elixir' },
  eelixir = { 'language', 'elixir', 'Embedded Elixir' },
  heex = { 'language', 'phoenix', 'Phoenix' },
  erlang = { 'language', 'erlang', 'Erlang' },
  fsharp = { 'language', 'fsharp', 'F#' },
  git = { 'language', 'git', 'Git' },
  gitattributes = { 'language', 'git', 'Git' },
  gitconfig = { 'language', 'git', 'Git' },
  gitignore = { 'language', 'git', 'Git' },
  gitsendemail = { 'language', 'git', 'Git' },
  go = { 'language', 'go', 'Go' },
  haskell = { 'language', 'haskell', 'Haskell' },
  html = { 'language', 'html', 'HTML' },
  java = { 'language', 'java', 'Java' },
  javascript = { 'language', 'javascript', 'JavaScript' },
  javascriptreact = { 'language', 'react', 'JSX' },
  json = { 'language', 'json', 'JSON' },
  julia = { 'language', 'julia', 'Julia' },
  tex = { 'language', 'latex', 'LaTeX' },
  texmf = { 'language', 'latex', 'LaTeX' },
  plaintex = { 'language', 'latex', 'LaTeX' },
  lisp = { 'language', 'lisp', 'Lisp' },
  lua = { 'language', 'lua', 'Lua' },
  nim = { 'language', 'nim', 'Nim' },
  nix = { 'language', 'nix', 'Nix' },
  ocaml = { 'language', 'ocaml', 'OCaml' },
  odin = { 'language', 'odin', 'Odin' },
  pascal = { 'language', 'pascal', 'Pascal' },
  perl = { 'language', 'perl', 'Perl' },
  php = { 'language', 'php', 'PHP' },
  ps1 = { 'language', 'powershell', 'PowerShell' },
  python = { 'language', 'python', 'Python' },
  quarto = { 'language', 'quarto', 'Quarto' },
  r = { 'language', 'r', 'R ' },
  ruby = { 'language', 'ruby', 'Ruby' },
  rust = { 'language', 'rust', 'Rust' },
  scala = { 'language', 'scala', 'Scala' },
  sass = { 'language', 'scss', 'Sass' },
  scss = { 'language', 'scss', 'Sass' },
  sh = { 'language', 'shell', 'Shell script' },
  sql = { 'language', 'sql', 'SQL' },
  svelte = { 'language', 'svelte', 'Svelte' },
  swift = { 'language', 'swift', 'Swift' },
  typescript = { 'language', 'typescript', 'TypeScript' },
  typescriptreact = { 'language', 'react', 'TSX' },
  v = { 'language', 'v', 'V ' },
  vim = { 'language', 'viml', 'VimL' },
  vue = { 'language', 'vue', 'Vue' },
  yaml = { 'language', 'yaml', 'YAML' },
  zig = { 'language', 'zig', 'Zig' },
  zsh = { 'language', 'shell', 'Zsh' },

  -- File Browsers
  netrw = { 'file_browser', M.default_icons.file_browser, 'Netrw' },
  TelescopePrompt = { 'file_browser', 'telescope', 'Telescope' },
  dirvish = { 'file_browser', M.default_icons.file_browser, 'Dirvish' },
  fern = { 'file_browser', M.default_icons.file_browser, 'Fern' },
  oil = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  oil_preview = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  oil_progress = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  NvimTree = { 'file_browser', M.default_icons.file_browser, 'nvim-tree' },
  minifiles = { 'file_browser', M.default_icons.file_browser, 'mini.files' },
  yazi = { 'file_browser', M.default_icons.file_browser, 'Yazi' },
  ['neo-tree'] = { 'file_browser', M.default_icons.file_browser, 'Neo-Tree' },

  -- Plugin Managers
  lazy = { 'plugin_manager', M.default_icons.plugin_manager, 'Lazy' },
  pckr = { 'plugin_manager', M.default_icons.plugin_manager, 'Packer' },
  packer = { 'plugin_manager', M.default_icons.plugin_manager, 'Packer' },

  -- LSP Managers
  mason = { 'lsp', M.default_icons.lsp, 'Mason' },
  lspinfo = { 'lsp', M.default_icons.lsp, 'LSP Info' },

  -- Docs
  help = { 'docs', M.default_icons.docs, 'Vim documentation' },
  help_ru = { 'docs', M.default_icons.docs, 'Vim documentation' },
  man = { 'docs', M.default_icons.docs, 'Man pages' },

  -- VCS
  magit = { 'vcs', M.default_icons.vcs, 'Magit' },
  gitcommit = { 'vcs', M.default_icons.vcs, 'Git' },
  gitrebase = { 'vcs', M.default_icons.vcs, 'Git' },
  fugitive = { 'vcs', M.default_icons.vcs, 'Fugitive' },
  fugitiveblame = { 'vcs', M.default_icons.vcs, 'Fugitive' },
  lazygit = { 'vcs', M.default_icons.vcs, 'Lazygit' },
  NeogitCommitSelectView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitCommitView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitConsole = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitDiffView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitGitCommandHistory = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitLogView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitPopup = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitRefsView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitReflogView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitStashView = { 'vcs', M.default_icons.vcs, 'Neogit' },
  NeogitStatus = { 'vcs', M.default_icons.vcs, 'Neogit' },
  ['git.nvim'] = { 'vcs', M.default_icons.vcs, 'Git.nvim' },

  -- Notes
  norg = { 'notes', 'neorg', 'Neorg' },
  org = { 'notes', 'org', 'Orgmode' },
  ['org-roam-node-buffer'] = { 'notes', 'org', 'Orgmode' },
  ['org-roam-select'] = { 'notes', 'org', 'Orgmode' },

  -- Dashboard
  alpha = { 'dashboard', M.default_icons.dashboard, 'Alpha' },
  dashboard = { 'dashboard', M.default_icons.dashboard, 'Dashboard' },
  dashboardpreview = { 'dashboard', M.default_icons.dashboard, 'Dashboard' },
  ministarter = { 'dashboard', M.default_icons.dashboard, 'mini.starter' },
  snacks_dashboard = { 'dashboard', M.default_icons.dashboard, 'Snacks Dashboard' },
  startify = { 'dashboard', M.default_icons.dashboard, 'Startify' },
}

M.special_cases = {
  -- Languages
  groovy = function(filename)
    return filename == 'build.gradle' and { 'language', 'gradle', 'Gradle' }
      or { 'language', 'groovy', 'Groovy' }
  end,
  kotlin = function(filename)
    return filename == 'build.gradle.kts' and { 'language', 'gradle', 'Gradle' }
      or { 'language', 'kotlin', 'Kotlin' }
  end,
  markdown = function(filename)
    return filename:lower() == 'license.md' and { 'language', 'license', 'License file' }
      or { 'language', 'markdown', 'Markdown' }
  end,
  text = function(filename)
    local name = filename:lower()
    return (name == 'license' or name == 'license.txt')
        and { 'language', 'license', 'License file' }
      or { 'language', M.default_icon, 'Plain Text' }
  end,
  toml = function(filename)
    return filename == 'Cargo.toml' and { 'language', 'cargo', 'Cargo' }
      or { 'language', 'toml', 'TOML' }
  end,
  xml = function(filename)
    return filename == 'pom.xml' and { 'language', 'maven', 'Maven' }
      or { 'language', 'xml', 'XML' }
  end,
}

M.extension_mappings = {
  -- Languages
  gml = { 'language', 'gml', 'GameMaker Language' },
  hx = { 'language', 'haxe', 'Haxe' },
  hxml = { 'language', 'haxe', 'Haxe' },
  pcss = { 'language', 'postcss', 'PostCSS' },
  postcss = { 'language', 'postcss', 'PostCSS' },
}

M.cord_related = {
  ['Cord.new'] = { 'language', M.default_icons.language, 'New file' },
  ['Cord.unknown'] = { 'language', M.default_icons.language, 'Unknown' },
}

M.get_default_icon = function(type) return M.default_icons[type] or M.default_icons.language end

M.get = function(filetype, filename)
  local result = M.mappings[filetype]
  if result then return result[1], result[2], result[3] end

  local result = M.special_cases[filetype]
  if result then
    local result = result(filename)
    return result[1], result[2], result[3]
  end

  if not config.advanced.plugin.match_in_mappings then return end
  local result = filename:match '%.([^%.]+)$'
  if result then
    result = M.extension_mappings[result:lower()]
    if result then return result[1], result[2], result[3] end
  end

  local result = M.cord_related[filetype]
  if result then return result[1], result[2], result[3] end
end

return M
