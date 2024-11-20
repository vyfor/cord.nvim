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
- Offers more than 70 icons for languages and components, each with custom design
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
    build = './build || .\\build',
    event = 'VeryLazy',
    opts = {}, -- calls require('cord').setup()
  }
  ```
</details>

<details>
  <summary>pckr.nvim</summary>

  ```lua
  {
    'vyfor/cord.nvim',
    run = './build || .\\build',
    config = function()
      require('cord').setup()
    end,
  }
  ```
</details>

<details>
  <summary>other</summary>
  <p>Same steps apply to other plugin managers. Just make sure to run <code>build.sh</code> or <code>build.bat</code> (depending on your OS) after the plugin is loaded</p>
</details>

## 🔧 Configuration
> Note: `setup()` has to be called to initialize the plugin.
```lua
require('cord').setup {
  usercmds = true,                              -- Enable user commands
  log_level = 'error',                          -- One of 'trace', 'debug', 'info', 'warn', 'error', 'off'
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
    swap_icons = false,                         -- If enabled, editor is displayed on the main image
    workspace_blacklist = {},                   -- List of workspace names that will hide rich presence
  },
  lsp = {
    show_problem_count = false,                 -- Display number of diagnostics problems
    severity = 1,                               -- 1 = Error, 2 = Warning, 3 = Info, 4 = Hint
    scope = 'workspace',                        -- buffer or workspace
  },
  idle = {
    enable = true,                              -- Enable idle status
    show_status = true,                         -- Display idle status, disable to hide the rich presence on idle
    timeout = 300000,                           -- Timeout in milliseconds after which the idle status is set, 0 to display immediately
    disable_on_focus = false,                   -- Do not display idle status when neovim is focused
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
  assets = nil,                                 -- Custom file icons, see the wiki*
  -- assets = {
  --   lazy = {                                 -- Vim filetype or file name or file extension = table or string
  --     name = 'Lazy',                         -- Optional override for the icon name, redundant for language types
  --     icon = 'https://example.com/lazy.png', -- Rich Presence asset name or URL
  --     tooltip = 'lazy.nvim',                 -- Text to display when hovering over the icon
  --     type = 'plugin_manager',               -- One of 'language', 'file_browser', 'plugin_manager', 'lsp_manager', 'vcs' or respective ordinals; defaults to 'language'
  --   },
  --   ['Cargo.toml'] = 'crates',
  -- },
}
```

<details>
  <summary>hide comments</summary>

  ```lua
  {
    usercmds = true,
    log_level = 'error',
    timer = {
      interval = 1500,
      reset_on_idle = false,
      reset_on_change = false,
    },
    editor = {
      image = nil,
      client = 'neovim',
      tooltip = 'The Superior Text Editor',
    },
    display = {
      show_time = true,
      show_repository = true,
      show_cursor_position = false,
      swap_fields = false,
      swap_icons = false,
      workspace_blacklist = {},
    },
    lsp = {
      show_problem_count = false,
      severity = 1,
      scope = 'workspace',
    },
    idle = {
      enable = true,
      show_status = true,
      timeout = 300000,
      disable_on_focus = false,
      text = 'Idle',
      tooltip = '💤',
    },
    text = {
      viewing = 'Viewing {}',
      editing = 'Editing {}',
      file_browser = 'Browsing files in {}',
      plugin_manager = 'Managing plugins in {}',
      lsp_manager = 'Configuring LSP in {}',
      vcs = 'Committing changes in {}',
      workspace = 'In {}',
    },
    buttons = {
      {
        label = 'View Repository',
        url = 'git',
      },
    },
    assets = nil,
  }
  ```
</details>

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
This project is in beta. Contributions of any kind are welcome, just make sure you read the [Contribution Guidelines](./.github/CONTRIBUTING.md) first. You can also contact me directly on Discord (**[vyfor](https://discord.com/users/446729269872427018)**) if you have any questions.

[**❓ FAQ**](https://github.com/vyfor/cord.nvim/wiki/FAQ)

[**🔧 Troubleshooting**](https://github.com/vyfor/cord.nvim/wiki/Troubleshooting)
