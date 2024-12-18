# 📖 Migration Guide

### **⚡ Cord - Tailor your Rich Presence like never before**

The plugin in question, **cord.nvim**, has been rewritten from scratch with a new client-server architecture in mind, bringing significant improvements in reliability and customization. Here's what you need to know when upgrading.

## ✨ What's New

- We have switched to Cargo for building instead of directly calling rustc in build scripts, which often caused inconsistencies. But don't worry, the dependency count is still at [zero](../Cargo.toml)!

- The plugin now runs as a server that handles all your Neovim instances. When you start Neovim, it launches a server in the background and connects to it. Any additional Neovim instances will connect to the same server, ensuring only one Rich Presence is shown. The most recent activity always takes priority.

- A new smart idle system has been implemented - you're only shown as idle when all your Neovim instances are actually idle. When an instance goes idle, it automatically switches back to show your most recent active one.

- The plugin is now event-driven, meaning changes are reflected instantly without any polling delays. When all instances disconnect, the server, as well as the connection to Discord, stay alive for a minute (configurable) before shutting down, which helps to avoid rate limiting issues.

- A new `variables` option allows users to define custom variables, including functions, for dynamic text templates. This enhances the flexibility and customization of the Rich Presence display.

- Many of the icons have been redesigned, and a new icon style has been introduced. The previous style, now called "Pastel" has been replaced with the default style, "Onyx" which features a sleek and modern dark theme. You can switch to a different style by using the `display.style` option.

> [!IMPORTANT]
> A nightly version of the Rust compiler is required to build the server component. The latest version of `rustup` can be downloaded from [here](https://rustup.rs/). Then, install the nightly toolchain using `rustup install nightly`.
> If you're using a plugin manager such as lazy.nvim, set the `build` key to `cargo build --release` to automatically rebuild the server on plugin updates.

## 🔧 Configuration Changes

The config structure has been updated to be more flexible. Most notably, the majority of string options now support functions, giving you full control over the Rich Presence display. Additionally, a new `variables` option has been introduced to allow custom dynamic values in text templates.

> [!NOTE]
> Full configuration options can be found [here](CONFIGURATION.md).

### Changed Options
```md
# [Old]                  # [New]
  timer.interval           Removed (now event-driven)
  timer.reset_on_idle      timestamp.reset_on_idle
  timer.reset_on_change    timestamp.reset_on_change
  editor.image             editor.icon
  display.show_time        timestamp.enabled
  idle.enable              idle.enabled
  idle.text                idle.details
  idle.disable_on_focus    idle.ignore_focus (inverted)
  log_level                use vim.log.levels instead of string values
```

### New Options
```lua
display = {
    style = 'onyx',                             -- Choose between 'onyx' (dark) or 'pastel' (accent)
}

idle = {
    smart_idle = true,                          -- Enable smart idle feature
    ignore_focus = true,                        -- Ignore window focus for idle state
}

variables = {}                                  -- Define custom variables for use in string templates

hooks = {
    on_ready = function() end,                  -- Server connection established
    on_update = function(opts) end,             -- Before building the activity
    on_activity = function(opts, activity) end, -- Before sending the activity
    on_idle = function(opts) end,               -- Entered idle state
    on_workspace_change = function(opts) end,   -- Workspace directory changed
    on_disconnect = function() end,             -- Server disconnected
}

advanced = {
    server = {
        pipe_path = nil,                        -- Custom IPC pipe path for the server
        executable_path = nil,                  -- Custom server executable path
        timeout = 60000,                        -- Server shutdown timeout (ms)
    },
    cursor_update_mode = 'on_move',             -- Which autocmd to use for cursor updates. One of 'on_move' or 'on_hold' or 'none'
}
```

### Removed Features
Several built-in features have been removed in favor of customization through functions:
- Workspace blacklist
- Cursor position display
- Problem count
- ToggleTerm handling

These can now be implemented using hooks and custom functions.

## 🎨 Function-Based Customization

The new version moves most of the functionality handled in Rust, to the Lua side, giving you unprecedented control over your Rich Presence. Almost every string option can now be a function that receives contextual information:

```lua
text = {
    -- Example of dynamic text based on file type
    editing = function(opts)
        if opts.filetype == 'rust' then
            return '🦀 Crafting in Rust'
        end
        return 'Editing ' .. opts.filename
    end,

    -- New types of mappings
    docs = 'Reading docs', -- Shown when in docs buffers
    dashboard = 'Home',    -- Shown when in dashboard buffers
}

-- Modify buttons dynamically
buttons = {
    {
        label = function(opts)
            if opts.git_url then
                return 'View Repository'
            end
            return 'My Website'
        end,
        url = function(opts)
            return opts.git_url or 'https://example.com'
        end
    }
}

-- New 'text' option that will override the entire text
assets = {
    ['Cargo.toml'] = {
        text = 'Managing dependencies' -- As opposed to 'Editing Cargo.toml'
    }
}```