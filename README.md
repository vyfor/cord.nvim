<div align="center">
  <h1>🧩 <strong>Cord</strong></h1>
  <div>
    <a href="https://github.com/vyfor/cord.nvim/stargazers"><img src="https://img.shields.io/github/stars/vyfor/cord.nvim?style=for-the-badge" alt="Stargazers"></a>
    <a href="https://github.com/vyfor/cord.nvim/blob/master/LICENSE"><img src="https://img.shields.io/github/license/vyfor/cord.nvim?style=for-the-badge" alt="Apache-2.0 License"></a>
    <a href="https://github.com/vyfor/cord.nvim/forks"><img src="https://img.shields.io/github/forks/vyfor/cord.nvim?style=for-the-badge" alt="Forks"></a>
  </div>
  <h3>🚀 <strong>Cord</strong> is a Discord Rich Presence plugin designed for Neovim, written in Rust.</h3>
  <img src="https://github.com/vyfor/cord.nvim/assets/92883017/d2e46243-2bef-4c73-bb3f-6d10edc2a2f4" alt="Cord Banner">
</div>

## 💎 Features
- Lightweight and dependency-free
- Cross-platform support (Windows, Linux, macOS)
- Blazingly fast startup with minimal overhead due to asynchronous nature
- Highly [configurable](#-configuration) in Lua
- Offers more than 60 icons for languages and components
- Automatically detects working directory and repository based on VCS
- Identifies problems across active buffers
- Supports configurable idle status detection
- Provides [user commands](#%EF%B8%8F-user-commands) for managing the presence
- Is written in native code and uses Lua FFI for integration

## 🔌 Requirements
- **Neovim >= 0.5.0 (compiled with LuaJIT)**
- **[Rust compiler](https://www.rust-lang.org/tools/install)**

## 📦 Installation
<details>
  <summary>lazy.nvim</summary>

  ```lua
  {
    'vyfor/cord.nvim',
    build = './build',
    event = 'VeryLazy',
    opts = {},
  }
  ```
</details>

<details>
  <summary>pckr.nvim</summary>

  ```lua
  {
    'vyfor/cord.nvim',
    run = './build',
  }
  ```
</details>

<details>
  <summary>other</summary>
  <p>Same steps apply to other plugin managers. Just make sure to add or run this build command:</p>

  ```sh
  ./build
  ```
</details>

> [!NOTE] 
> If you are on Windows Command Prompt, you may need to use a backslash as the path separator: `build = '.\\build'`

## 🔧 Configuration
> Note: `setup()` has to be called to initialize the plugin.
```lua
require('cord').setup({
  usercmds = true,                              -- Enable user commands
  timer = {
    interval = 1500,                            -- Interval between presence updates in milliseconds (min 500)
    reset_on_idle = false,                      -- Reset start timestamp on idle
    reset_on_change = false,                    -- Reset start timestamp on presence change
  },
  editor = {
    image = nil,                                -- Image ID or URL in case a custom client id is provided
    client = 'neovim',                          -- vim, neovim, lunarvim, nvchad, astronvim or your application's client id
    tooltip = 'The Superior Text Editor',       -- Text to display when hovering over the editor's image
  },
  display = {
    show_time = true,                           -- Display start timestamp
    show_repository = true,                     -- Display 'View repository' button linked to repository url, if any
    show_cursor_position = false,               -- Display line and column number of cursor's position
    swap_fields = false,                        -- If enabled, workspace is displayed first
    workspace_blacklist = {},                   -- List of workspace names to hide
  },
  lsp = {
    show_problem_count = false,                 -- Display number of diagnostics problems
    severity = 1,                               -- 1 = Error, 2 = Warning, 3 = Info, 4 = Hint
    scope = 'workspace',                        -- buffer or workspace
  },
  idle = {
    enable = true,                              -- Enable idle status
    show_status = true,                         -- Display idle status, disable to hide the rich presence on idle
    timeout = 1800000,                          -- Timeout in milliseconds after which the idle status is set, 0 to display immediately
    disable_on_focus = true,                    -- Do not display idle status when neovim is focused
    text = 'Idle',                              -- Text to display when idle
    tooltip = '💤',                             -- Text to display when hovering over the idle image
  },
  text = {
    viewing = 'Viewing {}',                     -- Text to display when viewing a readonly file
    editing = 'Editing {}',                     -- Text to display when editing a file
    file_browser = 'Browsing files in {}',      -- Text to display when browsing files (Empty string to disable)
    plugin_manager = 'Managing plugins in {}',  -- Text to display when managing plugins (Empty string to disable)
    lsp_manager = 'Configuring LSP in {}',      -- Text to display when managing LSP servers (Empty string to disable)
    vcs = 'Committing changes in {}',           -- Text to display when using Git or Git-related plugin (Empty string to disable)
    workspace = 'In {}',                        -- Text to display when in a workspace (Empty string to disable)
  },
  buttons = {
    {
      label = 'View Repository',                -- Text displayed on the button
      url = 'git',                              -- URL where the button leads to ('git' = automatically fetch Git repository URL)
    },
    -- {
    --   label = 'View Plugin',
    --   url = 'https://github.com/vyfor/cord.nvim',
    -- }
  },
  assets = {                                    -- Custom file icons
    -- lazy = {                                 -- Vim filetype or file name or file extension = table or string (see wiki)*
    --   name = 'Lazy',                         -- Optional override for the icon name, redundant for language types
    --   icon = 'https://example.com/lazy.png', -- Rich Presence asset name or URL
    --   tooltip = 'lazy.nvim',                 -- Text to display when hovering over the icon
    --   type = 2,                              -- 0 = language, 1 = file browser, 2 = plugin manager, 3 = lsp manager, 4 = vcs; defaults to language
    -- },
    -- ['Cargo.toml'] = 'crates',
  },
})
```
> \* [Wiki: Add or change file icons](https://github.com/vyfor/cord.nvim/wiki/Add-or-change-file-icons)

### ⌨️ User commands
- `:CordConnect`          - Initialize presence client internally and connect to Discord
- `:CordReconnect`        - Reconnect to Discord
- `:CordDisconnect`       - Disconnect from Discord
- `:CordTogglePresence`   - Toggle presence
- `:CordShowPresence`     - Show presence
- `:CordHidePresence`     - Hide presence
- `:CordToggleIdle`       - Toggle idle status
- `:CordIdle`             - Show idle status
- `:CordUnidle`           - Hide idle status and reset the timeout
- `:CordWorkspace <name>` - Change the name of the workspace (visually)

## 🌱 Contributing
This project is in beta. Feel free to open an issue or pull request for missing icons or features. You can also contact me on Discord (**[vyfor](https://discord.com/users/446729269872427018)**) if you have any questions.
