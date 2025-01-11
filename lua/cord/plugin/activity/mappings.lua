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
  debug = 'bug',
  test = 'tests',
  diagnostics = 'diagnostics',
  games = 'controller',
  dashboard = 'dashboard',
}

M.filetype_mappings = {
  -- Languages
  ada = { 'language', 'ada', 'Ada' },
  autohotkey = { 'language', 'ahk', 'AutoHotkey' },
  automake = { 'language', 'gnu', 'Automake' },
  asm = { 'language', 'assembly', 'Assembly' },
  asm68k = { 'language', 'assembly', 'Assembly' },
  asmh8300 = { 'language', 'assembly', 'Assembly' },
  astro = { 'language', 'astro', 'Astro' },
  awk = { 'language', 'awk', 'Awk' },
  c = { 'language', 'c', 'C ' },
  clojure = { 'language', 'clojure', 'Clojure' },
  conf = { 'language', 'gear', 'Config' },
  config = { 'language', 'gear', 'Config' },
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
  git = { 'language', 'git', 'Git' },
  gitattributes = { 'language', 'git', 'Git' },
  gitconfig = { 'language', 'git', 'Git' },
  gitignore = { 'language', 'git', 'Git' },
  gitsendemail = { 'language', 'git', 'Git' },
  go = { 'language', 'go', 'Go' },
  groovy = { 'language', 'groovy', 'Groovy' },
  haskell = { 'language', 'haskell', 'Haskell' },
  html = { 'language', 'html', 'HTML' },
  hyprlang = { 'language', 'hyprland', 'Hyprland' },
  java = { 'language', 'java', 'Java' },
  javascript = { 'language', 'javascript', 'JavaScript' },
  javascriptreact = { 'language', 'react', 'JSX' },
  json = { 'language', 'json', 'JSON' },
  json5 = { 'language', 'json', 'JSON5' },
  jsonc = { 'language', 'json', 'JSON' },
  julia = { 'language', 'julia', 'Julia' },
  kotlin = { 'language', 'kotlin', 'Kotlin' },
  tex = { 'language', 'latex', 'LaTeX' },
  texmf = { 'language', 'latex', 'LaTeX' },
  text = { 'language', M.default_icons.language, 'Plain Text' },
  toml = { 'language', 'toml', 'TOML' },
  plaintex = { 'language', 'latex', 'LaTeX' },
  lisp = { 'language', 'lisp', 'Lisp' },
  logcheck = { 'language', 'logs', 'Logcheck' },
  lua = { 'language', 'lua', 'Lua' },
  make = { 'language', 'gnu', 'Makefile' },
  markdown = { 'language', 'markdown', 'Markdown' },
  matlab = { 'language', 'matlab', 'MATLAB' },
  nim = { 'language', 'nim', 'Nim' },
  nix = { 'language', 'nix', 'Nix' },
  obj = { 'language', 'assembly', 'Object' },
  objc = { 'language', 'c', 'Objective-C' },
  objcpp = { 'language', 'cpp', 'Objective-C++' },
  objdump = { 'language', 'assembly', 'Object Dump' },
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
  svg = { 'language', 'svg', 'SVG' },
  swift = { 'language', 'swift', 'Swift' },
  typescript = { 'language', 'typescript', 'TypeScript' },
  typescriptreact = { 'language', 'react', 'TSX' },
  v = { 'language', 'v', 'V ' },
  vim = { 'language', 'viml', 'VimL' },
  viminfo = { 'language', 'viml', 'VimL' },
  vue = { 'language', 'vue', 'Vue' },
  winbatch = { 'language', 'shell', 'Batch' },
  xml = { 'language', 'xml', 'XML' },
  yaml = { 'language', 'yaml', 'YAML' },
  zig = { 'language', 'zig', 'Zig' },
  zsh = { 'language', 'shell', 'Zsh' },

  -- File Browsers
  carbon = { 'file_browser', M.default_icons.file_browser, 'Carbon' },
  CHADTree = { 'file_browser', M.default_icons.file_browser, 'ChadTree' },
  dirbuf = { 'file_browser', M.default_icons.file_browser, 'Dirbuf' },
  dirvish = { 'file_browser', M.default_icons.file_browser, 'Dirvish' },
  drex = { 'file_browser', M.default_icons.file_browser, 'Drex' },
  fern = { 'file_browser', M.default_icons.file_browser, 'Fern' },
  Fm = { 'file_browser', M.default_icons.file_browser, 'Fm' },
  fzf = { 'file_browser', M.default_icons.file_browser, 'FZF' },
  fzflua_backdrop = { 'file_browser', M.default_icons.file_browser, 'FZF' },
  lir = { 'file_browser', M.default_icons.file_browser, 'Lir' },
  oil = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  oil_preview = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  oil_progress = { 'file_browser', M.default_icons.file_browser, 'Oil' },
  nerdtree = { 'file_browser', M.default_icons.file_browser, 'NerdTree' },
  NNN = { 'file_browser', M.default_icons.file_browser, 'NNN' },
  NvimTree = { 'file_browser', M.default_icons.file_browser, 'nvim-tree' },
  minifiles = { 'file_browser', M.default_icons.file_browser, 'mini.files' },
  rnvimr = { 'file_browser', M.default_icons.file_browser, 'Ranger' },
  sfm = { 'file_browser', M.default_icons.file_browser, 'SFM' },
  TelescopePrompt = { 'file_browser', 'telescope', 'Telescope' },
  tfm = { 'file_browser', M.default_icons.file_browser, 'TFM' },
  triptych_backdrop = { 'file_browser', M.default_icons.file_browser, 'Triptych' },
  Yanil = { 'file_browser', M.default_icons.file_browser, 'Yanil' },
  yazi = { 'file_browser', M.default_icons.file_browser, 'Yazi' },
  ['neo-tree'] = { 'file_browser', M.default_icons.file_browser, 'Neo-Tree' },

  -- Plugin Managers
  lazy = { 'plugin_manager', M.default_icons.plugin_manager, 'Lazy' },
  pckr = { 'plugin_manager', M.default_icons.plugin_manager, 'Packer' },
  packer = { 'plugin_manager', M.default_icons.plugin_manager, 'Packer' },
  ['minideps-confirm'] = { 'plugin_manager', M.default_icons.plugin_manager, 'mini.deps' },

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
  note = { 'notes', M.default_icons.notes, 'Note.nvim' },
  org = { 'notes', 'org', 'Orgmode' },
  ['org-roam-node-buffer'] = { 'notes', 'org', 'Orgmode' },
  ['org-roam-select'] = { 'notes', 'org', 'Orgmode' },

  -- Dashboard
  alpha = { 'dashboard', M.default_icons.dashboard, 'Alpha' },
  dashboard = { 'dashboard', M.default_icons.dashboard, 'Dashboard' },
  dashboardpreview = { 'dashboard', M.default_icons.dashboard, 'Dashboard' },
  ministarter = { 'dashboard', M.default_icons.dashboard, 'mini.starter' },
  nvdash = { 'dashboard', M.default_icons.dashboard, 'NvDash' },
  profile = { 'dashboard', M.default_icons.dashboard, 'Profile' },
  snacks_dashboard = { 'dashboard', M.default_icons.dashboard, 'Snacks Dashboard' },
  spaceport = { 'dashboard', M.default_icons.dashboard, 'Spaceport' },
  startify = { 'dashboard', M.default_icons.dashboard, 'Startify' },
  startup = { 'dashboard', M.default_icons.dashboard, 'Startup' },
  veil = { 'dashboard', M.default_icons.dashboard, 'Veil' },
}

M.filename_mappings = {
  -- Languages
  ['.gemignore'] = { 'language', 'rubygems', 'RubyGems' },
  ['.npmrc'] = { 'language', 'npm', 'npm' },
  ['.npmignore'] = { 'language', 'npm', 'npm' },
  ['build.gradle'] = { 'language', 'gradle', 'Gradle' },
  ['build.gradle.kts'] = { 'language', 'gradle', 'Gradle' },
  ['cargo.toml'] = { 'language', 'cargo', 'Cargo' },
  ['gemfile'] = { 'language', 'rubygems', 'RubyGems' },
  ['license'] = { 'language', 'license', 'License file' },
  ['license.md'] = { 'language', 'license', 'License file' },
  ['license.txt'] = { 'language', 'license', 'License file' },
  ['package.json'] = { 'language', 'npm', 'npm' },
  ['pom.xml'] = { 'language', 'maven', 'Maven' },
  ['settings.gradle'] = { 'language', 'gradle', 'Gradle' },
  ['settings.gradle.kts'] = { 'language', 'gradle', 'Gradle' },
  ['vue.config.js'] = { 'language', 'vue', 'Vue' },
}

M.extension_mappings = {
  -- Languages
  ['.bmp'] = { 'language', 'picture', 'BMP' },
  ['.fs'] = { 'language', 'fsharp', 'F#' },
  ['.fsi'] = { 'language', 'fsharp', 'F#' },
  ['.fsscript'] = { 'language', 'fsharp', 'F#' },
  ['.fsx'] = { 'language', 'fsharp', 'F#' },
  ['.gem'] = { 'language', 'rubygems', 'RubyGems' },
  ['.gemspec'] = { 'language', 'rubygems', 'RubyGems' },
  ['.gd'] = { 'language', 'godot', 'Godot' },
  ['.godot'] = { 'language', 'godot', 'Godot' },
  ['.gif'] = { 'language', 'picture', 'GIF' },
  ['.gml'] = { 'language', 'gml', 'GameMaker Language' },
  ['.heif'] = { 'language', 'picture', 'HEIF' },
  ['.hx'] = { 'language', 'haxe', 'Haxe' },
  ['.hxml'] = { 'language', 'haxe', 'Haxe' },
  ['.ico'] = { 'language', 'picture', 'ICO' },
  ['.ipynb'] = { 'language', 'jupyter', 'Jupyter Notebook' },
  ['.jpeg'] = { 'language', 'picture', 'JPEG' },
  ['.jpg'] = { 'language', 'picture', 'JPEG' },
  ['.lock'] = { 'language', 'lock', 'Lockfile' },
  ['.log'] = { 'language', 'logs', 'Logs' },
  ['.module.ts'] = { 'language', 'angular', 'Angular' },
  ['.module.js'] = { 'language', 'angular', 'Angular' },
  ['.component.ts'] = { 'language', 'angular', 'Angular' },
  ['.component.js'] = { 'language', 'angular', 'Angular' },
  ['.component.html'] = { 'language', 'angular', 'Angular' },
  ['.component.css'] = { 'language', 'angular', 'Angular' },
  ['.component.scss'] = { 'language', 'angular', 'Angular' },
  ['.component.less'] = { 'language', 'angular', 'Angular' },
  ['.component.spec'] = { 'language', 'angular', 'Angular' },
  ['.service.ts'] = { 'language', 'angular', 'Angular' },
  ['.service.js'] = { 'language', 'angular', 'Angular' },
  ['.directive.ts'] = { 'language', 'angular', 'Angular' },
  ['.directive.js'] = { 'language', 'angular', 'Angular' },
  ['.pipe.ts'] = { 'language', 'angular', 'Angular' },
  ['.pipe.js'] = { 'language', 'angular', 'Angular' },
  ['.guard.ts'] = { 'language', 'angular', 'Angular' },
  ['.guard.js'] = { 'language', 'angular', 'Angular' },
  ['.interceptor.ts'] = { 'language', 'angular', 'Angular' },
  ['.interceptor.js'] = { 'language', 'angular', 'Angular' },
  ['.model.ts'] = { 'language', 'angular', 'Angular' },
  ['.model.js'] = { 'language', 'angular', 'Angular' },
  ['.interface.ts'] = { 'language', 'angular', 'Angular' },
  ['.interface.js'] = { 'language', 'angular', 'Angular' },
  ['.pcss'] = { 'language', 'postcss', 'PostCSS' },
  ['.png'] = { 'language', 'picture', 'PNG' },
  ['.postcss'] = { 'language', 'postcss', 'PostCSS' },
  ['.tiff'] = { 'language', 'picture', 'TIFF' },
  ['.webp'] = { 'language', 'picture', 'WebP' },
}

M.cord_related = {
  ['Cord.new'] = { 'language', M.default_icons.language, 'New file' },
  ['Cord.unknown'] = { 'language', M.default_icons.language, 'Unknown' },
}

M.get_default_icon = function(type) return M.default_icons[type] or M.default_icons.language end

M.get = function(filetype, filename)
  local result = M.filename_mappings[filename:lower()]
  if result then return result[1], result[2], result[3] end

  if config.advanced.plugin.match_in_mappings then
    local extension = filename:match '%..*$'
    if extension then
      result = M.extension_mappings[extension]
      if result then return result[1], result[2], result[3] end
    end
  end

  local result = M.filetype_mappings[filetype]
  if result then return result[1], result[2], result[3] end

  local result = M.cord_related[filetype]
  if result then return result[1], result[2], result[3] end
end

return M
