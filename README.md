# 🧩 **Cord**

### 🚀 **Cord** is a Discord Rich Presence plugin designed for Neovim, written in Rust.

![cord_banner](https://github.com/vyfor/cord.nvim/assets/92883017/6ff91794-7264-485e-b82b-87926d7d5013)


## 💎 Features
- Lightweight and dependency-free
- Cross-platform support (Windows, Linux, macOS)
- Blazingly fast startup due to non-blocking, asynchronous nature
- Highly [configurable](#-configuration) in Lua
- Offers a rich icon set for various components
- Automatically detects working directory and repository based on Git
- Identifies problems across active buffers
- Supports configurable idle status detection
- Provides [user commands](%EF%B8%8F-user-commands) for managing the presence
- Is written in native code and uses Lua FFI for integration

## 🔌 Requirements
- **Neovim compiled with LuaJIT**
- **[Rust compiler](https://www.rust-lang.org/tools/install)**

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
    enable = true,                              -- Enable automatically updating presence
    interval = 1500,                            -- Interval between presence updates in milliseconds (min 500)
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
    viewing = 'Viewing {}',                     -- Text to display when viewing a readonly file
    editing = 'Editing {}',                     -- Text to display when editing a file
    file_browser = 'Browsing files in {}',      -- Text to display when browsing files (Empty string to disable)
    plugin_manager = 'Managing plugins in {}',  -- Text to display when managing plugins (Empty string to disable)
    workspace = 'In {}',                        -- Text to display when in a workspace (Empty string to disable)
  },
  buttons = {
    {
      label = 'View repository',                -- Text displayed on the button
      url = 'git',                              -- URL where the button leads to ('git' = Git repository URL)
    },
    -- {
    --   label = 'View plugin',
    --   url = 'https://github.com/vyfor/cord.nvim',
    -- }
  }
})
```

### ⌨️ User commands
- `:CordConnect`        - Initialize presence client internally and connect to Discord
- `:CordReconnect`      - Reconnect to Discord
- `:CordDisconnect`     - Disconnect from Discord
- `:CordTogglePresence` - Toggle presence
- `:CordShowPresence`   - Show presence
- `:CordHidePresence`   - Hide presence

## 🌱 Contributing
This project is in beta. Feel free to open an issue or pull request for missing icons or features. You can also contact me on Discord **[vyfor](https://discord.com/users/446729269872427018)** if you have any questions.

## ❓ FAQ
### Why Rust?
> There are two primary reasons as to why the project has been rewritten in Rust. Firstly, the compilation process: Kotlin/Native, despite compiling to native code, still relies on the JVM for its compiler, which is inconvenient, aside from that, compile times are quite slow. Secondly, Rust is known for its performance and safety, which is why the change was made.

### Why was Lua not considered?
> The internal code needs to run on a separate thread due to Discord's ratelimit enforcement between connections. Implementing multithreading is much simpler in Rust compared to Lua. Although, a considerable part of the codebase still relies on Lua code.

### Why does Cord use a timer-based approach?
> Certain plugins, particularly file browser ones, tend to break the event sequence. Thus, it was decided to use a timer. Regardless of that, Cord continues to rely on autocommands for aspects less prone to change, such as workspace or Git repository.

### Dependency-free?
> Every aspect, including FFI, JSON serialization and pipe connection, is implemented from scratch to avoid reliance on external crates, simply to prevent any increase in compile times. Serialization is mainly hard-coded to improve performance, even by a negligible amount. 🤫
