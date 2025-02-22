# 📖 Migration Guide

### **⚡ Cord - Rich Presence Reimagined**

The plugin in question, **cord.nvim**, has been rewritten from scratch with a new client-server architecture in mind, bringing significant improvements in reliability and customization. Here's what you need to know when upgrading.

## ✨ What's New

- We have switched to Cargo for building instead of directly calling rustc in build scripts, which often caused inconsistencies. But don't worry, the dependency count is still at [zero](../Cargo.toml)! Plus, users can benefit from incremental builds.

- The plugin now runs as a server that handles all your Neovim instances. When you start Neovim, it launches a server in the background and connects to it. Any additional Neovim instances will connect to the same server, ensuring only one Rich Presence is shown. The most recent activity always takes priority.

- A new smart idle system has been implemented - you're only shown as idle when all your Neovim instances are actually idle. When an instance goes idle, it automatically switches back to show your most recent active one.

- The plugin is now event-driven, meaning changes are reflected instantly without any polling delays. When all instances disconnect, the server, as well as the connection to Discord, stay alive for a little while (configurable) before shutting down, which helps to avoid rate limiting issues.

- A new `variables` option allows users to define custom variables, including functions, for dynamic text templates. This enhances the flexibility and customization of the Rich Presence display.

- Many of the icons have been redesigned, and a new theme has been introduced. The previous style, now called "Pastel" has been replaced with the new default style, "Onyx" which features a sleek and modern dark theme. You can switch between themes by using the `display.theme` option.

> [!IMPORTANT]
> The Rust compiler is no longer required. Cord's server can now be downloaded from GitHub via the `:Cord update` command.
> However, if you wish to build Cord from source, make sure to have **Rust >= 1.85.0 nightly** installed, and run the `:Cord update build` command.

| v1 Option                      | v2 Option                      | Notes                                                               |
| ------------------------------ | ------------------------------ | ------------------------------------------------------------------- |
| `timer.interval`               | *(Removed - now event-driven)* | Presence updates are now real-time, no interval needed.             |
| `display.show_repository`      | *(Removed)*                    | Repository display is now handled through text customization.       |
| `display.show_cursor_position` | *(Removed)*                    | Cursor position is now handled through text customization.          |
| `display.workspace_blacklist`  | *(Removed)*                    | Workspace blacklisting is now handled through hooks/functions.      |
| `lsp`                          | *(Removed)*                    | LSP integration is now handled through plugins (e.g., diagnostics). |
| `usercmds`                     | *(Always available)*           | User commands are now always available under `:Cord <command>`.     |
| `timer`                        | `timestamp`                    | Renamed for clarity.                                                |
| `timer.enabled`                | `timestamp.enabled`            |                                                                     |
| `timer.reset_on_idle`          | `timestamp.reset_on_idle`      |                                                                     |
| `timer.reset_on_change`        | `timestamp.reset_on_change`    |                                                                     |
| `editor.image`                 | `editor.icon`                  | Renamed for consistency.                                            |
| `idle.enable`                  | `idle.enabled`                 |                                                                     |
| `idle.text`                    | `idle.details`                 | Renamed for clarity.                                                |
| `idle.disable_on_focus`        | `idle.ignore_focus`            | Inverted logic - now `ignore_focus` (more intuitive).               |

The biggest change in v2 is the shift towards function-based configuration.  **Almost every string option can now be a Lua function!** This unlocks powerful dynamic customization:

- **Dynamic Text**:  Use functions in `text` options to create context-aware presence messages based on filetype, workspace, time, and more (see [Examples](./Examples.md)).
- **Dynamic Buttons**:  Create buttons with labels and URLs that change based on the current Neovim state.
- **Dynamic Assets**:  Customize icons and asset text dynamically using functions in the `assets` table.
- **Hooks for Advanced Logic**:  Use hooks to execute custom Lua code at various points in Cord's update cycle, enabling complex integrations and behaviors.

**Example: Dynamic Editing Text based on Filetype**

```lua
text = {
    editing = function(opts)
        if opts.filetype == 'rust' then
            return '🦀 Crafting in Rust' -- Fun Rust-specific text
        else
            return 'Editing ' .. opts.filename -- Default editing text
        end
    end,
}
```

**Example: Dynamic Button Label**

```lua
buttons = {
    {
        label = function(opts)
            return opts.repo_url and 'View Repository' or 'View cord.nvim'
        end,
        url = function(opts)
            return opts.repo_url or 'https://github.com/vyfor/cord.nvim'
        end
    }
}
```

## 🚀 Upgrade Steps: Get Started with v2

1. **Update Cord Plugin**: Use your plugin manager to update `cord.nvim`.
2. **Run `:Cord update`**:  In Neovim, run `:Cord update` to download the new server executable.
3. **Review Your Configuration**:  Carefully review your `cord.setup()` configuration and update it to the v2 structure, taking into account the renamed and removed options.
4. **Explore Examples & Documentation**:  Check out the [Examples](./Examples.md) page and the [Configuration Guide](./Configuration.md) to learn about the new customization options and how to use functions and hooks.
5. **Test and Customize**:  Start using Cord v2 and customize your presence to your liking! Use `:checkhealth cord` to verify your configuration.

**Need Help?**

If you encounter any issues during migration or have questions, please don't hesitate to:

- **Check the [Troubleshooting Guide](./Troubleshooting.md)**
- **Join our [Discord Community](https://discord.gg/q9rC4bjCHv)**
- **Start a [Discussion on GitHub](https://github.com/vyfor/cord.nvim/discussions)**

Welcome to the future of Neovim Rich Presence with Cord v2! We're confident you'll love the enhanced power and flexibility. Happy coding!