# 📖 Migration Guide

### **⚡ Cord - Tailor your Rich Presence like never before**

The plugin in question, **cord.nvim**, has been rewritten from scratch with a new client-server architecture in mind, bringing significant improvements in reliability and customization. Here's what you need to know when upgrading.

## ✨ What's New

- We have switched to Cargo for building instead of directly calling rustc in build scripts, which often caused inconsistencies. But don't worry, the dependency count is still at [zero](../Cargo.toml)! Plus, users can benefit from incremental builds.

- The plugin now runs as a server that handles all your Neovim instances. When you start Neovim, it launches a server in the background and connects to it. Any additional Neovim instances will connect to the same server, ensuring only one Rich Presence is shown. The most recent activity always takes priority.

- A new smart idle system has been implemented - you're only shown as idle when all your Neovim instances are actually idle. When an instance goes idle, it automatically switches back to show your most recent active one.

- The plugin is now event-driven, meaning changes are reflected instantly without any polling delays. When all instances disconnect, the server, as well as the connection to Discord, stay alive for a minute (configurable) before shutting down, which helps to avoid rate limiting issues.

- A new `variables` option allows users to define custom variables, including functions, for dynamic text templates. This enhances the flexibility and customization of the Rich Presence display.

- Many of the icons have been redesigned, and a new theme has been introduced. The previous style, now called "Pastel" has been replaced with the new default style, "Onyx" which features a sleek and modern dark theme. You can switch between themes by using the `display.theme` option.

> [!IMPORTANT]
> A nightly version of the Rust compiler is required to build the server component. The latest version of `rustup` can be downloaded from [here](https://rustup.rs/). Then, specify the nightly version in the installation options or run `rustup install nightly` in the terminal.
> If you're using a plugin manager such as lazy.nvim, set the `build` key to `cargo build --release` to automatically rebuild the server executable on plugin updates.

## 🔧 Configuration Changes

The config structure has been updated to be more flexible. Most notably, the majority of string options now support functions, giving you full control over the Rich Presence display. Additionally, a new `variables` option has been introduced to allow custom dynamic values in text templates.

> [!NOTE]
> Full configuration options can be found [here](CONFIGURATION.md).

### Changed Options
```lua
-- Removed Options
-- timer.interval (now event-driven)
-- display.show_repository
-- display.show_cursor_position
-- display.workspace_blacklist
-- lsp
-- usercmds (now always available under 'Cord <command>')

-- Renamed Options
timestamp = {             -- was timer
  enabled = true,         -- was display.show_time
  reset_on_idle = true,   -- was timer.reset_on_idle
  reset_on_change = true, -- was timer.reset_on_change
}

editor = {
  icon = nil,             -- was editor.image
}

idle = {
  enabled = true,         -- was idle.enable
  details = 'Idling',     -- was idle.text
  ignore_focus = false,   -- was idle.disable_on_focus (inverted)
}

-- Moved Options
advanced = {
  plugin = {
    log_level = vim.log.levels.INFO, -- was log_level (now uses vim.log.levels)
  }
}
```

### Removed Features
Several built-in features have been removed in favor of customization through functions:
- Workspace blacklist
- Cursor position display
- Problem count
- ToggleTerm handling

These can now be implemented using hooks and custom functions. See [examples](EXAMPLES.md).

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
            if opts.repo_url then
                return 'View Repository'
            end
            return 'My Website'
        end,
        url = function(opts)
            return opts.repo_url or 'https://example.com'
        end
    }
}

-- New 'text' option that will override the entire text
assets = {
    ['Cargo.toml'] = {
        text = 'Managing dependencies' -- As opposed to 'Editing Cargo.toml'
    }
}
```

More information can be found in the [Configuration Guide](CONFIGURATION.md).