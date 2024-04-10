# 🧩 **Cord**

### 🚀 **Cord** is a Discord Rich Presence plugin designed for Neovim, written in Rust.

![cord_banner](https://github.com/vyfor/cord.nvim/assets/92883017/d4d3ef5c-a347-46ea-a1a8-24a7086ee47e)

## 💎 Features
- Lightweight
- Blazingly fast startup due to non-blocking, asynchronous nature
- Highly [configurable](https://github.com/vyfor/cord.nvim#-configuration) in Lua
- Offers a rich icon set for various components
- Automatically detects working directory and repository based on Git
- Identifies problems across active buffers
- Supports configurable idle status detection
- Is written in native code and uses Lua FFI for integration

## 🔌 Requirements
- **Neovim compiled with LuaJIT**
- **Rust compiler**

## 📦 Installation
<details>
  <summary>lazy.nvim</summary>

  ```lua
  {
    'vyfor/cord.nvim',
    build = './build'
  }
  ```
</details>

<details>
  <summary>pckr.nvim</summary>

  ```lua
  {
    'vyfor/cord.nvim',
    run = './build'
  }
  ```
</details>

<details>
  <summary>other</summary>
  <p>Same steps apply to other plugin managers. Just make sure to add/run this build command:</p>

  ```sh
  ./build
  ```
</details>

## 🔧 Configuration
```lua
require('cord').setup({
  usercmds = true,                              -- Enable user commands
  timer = {
    enable = true,                              -- Enable timer
    interval = 1500,                            -- Timer's update interval in milliseconds (min 500)
    reset_on_idle = false,                      -- Reset start timestamp on idle
    reset_on_change = false,                    -- Reset start timestamp on presence change
  },
  editor = {
    image = nil,                                -- Image ID or URL in case a custom client id is provided
    client = 'neovim',                          -- vim, neovim, lunarvim, nvchad or your application's client id
    tooltip = 'The Superior Text Editor',       -- Text to display when hovering over the editor's image
  },
  display = {
    show_time = true,                           -- Display start timestamp
    show_repository = true,                     -- Display 'View repository' button linked to repository url, if any
    show_cursor_position = true,                -- Display line and column number of cursor's position
  },
  lsp = {
    show_problem_count = false,                 -- Display number of diagnostics problems
    severity = 1,                               -- 1 = Error, 2 = Warning, 3 = Info, 4 = Hint
    scope = 'workspace',                        -- buffer or workspace
  }
  idle = {
    show_idle = true,                           -- Enable idle status
    timeout = 300000,                           -- Timeout in milliseconds after which the idle status is set, 0 to display immediately
    disable_on_focus = true,                    -- Do not display idle status when neovim is focused
    text = 'Idle',                              -- Text to display when idle
    tooltip = '💤',                             -- Text to display when hovering over the idle image
  },
  text = {
    viewing = 'Viewing $s',                     -- Text to display when viewing a readonly file
    editing = 'Editing $s',                     -- Text to display when editing a file
    file_browser = 'Browsing files in $s',      -- Text to display when browsing files (Empty string to disable)
    plugin_manager = 'Managing plugins in $s',  -- Text to display when managing plugins (Empty string to disable)
    workspace = 'In $s',                        -- Text to display when in a workspace (Empty string to disable)
  }
})
```

### ⌨️ User commands (WIP)
- `:CordConnect`        - Initialize presence client internally and connect to Discord
- `:CordReconnect`      - Reconnect to Discord
- `:CordDisconnect`     - Disconnect from Discord
- `:CordTogglePresence` - Toggle presence
- `:CordShowPresence`   - Show presence
- `:CordHidePresence`   - Hide presence

## 🌱 Contributing
This project is in beta. Feel free to open an issue or pull request for missing icons or features. You can also contact me on Discord **[vyfor](https://discord.com/users/446729269872427018)** if you have any questions.

