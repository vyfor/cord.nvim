# ⚡ Cord Configuration

A comprehensive guide to configuring the Cord plugin to your liking. All options are set through the `setup()` function:

```lua
require('cord').setup {
    -- your options here
}
```

## 🔧 Core Options

| Option      | Type      | Default                | Description                                                  |
| ----------- | --------- | ---------------------- | ------------------------------------------------------------ |
| `usercmds`  | `boolean` | `true`                 | Enable [user commands](#user-commands) for managing presence |
| `log_level` | `number`  | `vim.log.levels.ERROR` | Set logging verbosity using Neovim's log levels              |

## ⏰ Timestamp

| Option                      | Type      | Default | Description                              |
| --------------------------- | --------- | ------- | ---------------------------------------- |
| `timestamp.enabled`         | `boolean` | `true`  | Show elapsed time in presence            |
| `timestamp.reset_on_idle`   | `boolean` | `false` | Reset timestamp when entering idle state |
| `timestamp.reset_on_change` | `boolean` | `false` | Reset timestamp when presence changes    |

## 🎨 Editor

| Option           | Type            | Default                      | Description                                                                                                                |
| ---------------- | --------------- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `editor.client`  | `string`        | `'neovim'`                   | Client identifier. Can be `'vim'`, `'neovim'`, `'lunarvim'`, `'nvchad'`, `'astronvim'`, or a custom Discord application ID |
| `editor.tooltip` | `string`        | `'The Superior Text Editor'` | Tooltip shown when hovering over editor icon                                                                               |
| `editor.icon`    | `string \| nil` | `nil`                        | Custom icon URL or asset ID when using custom client ID                                                                    |

## 📊 Display

| Option                | Type      | Default | Description                         |
| --------------------- | --------- | ------- | ----------------------------------- |
| `display.swap_fields` | `boolean` | `false` | Show workspace name before filename |
| `display.swap_icons`  | `boolean` | `false` | Use editor icon as large image      |

## 💤 Idle

| Option              | Type                       | Default    | Description                                                   |
| ------------------- | -------------------------- | ---------- | ------------------------------------------------------------- |
| `idle.enabled`      | `boolean`                  | `true`     | Enable idle status detection                                  |
| `idle.timeout`      | `number`                   | `300000`   | Milliseconds before marking the session as idle               |
| `idle.show_status`  | `boolean`                  | `true`     | Show idle status in presence, or hide the presence if `false` |
| `idle.ignore_focus` | `boolean`                  | `true`     | Show idle when Neovim is focused                              |
| `idle.smart_idle`   | `boolean`                  | `true`     | Enable [smart idle](#smart-idle) feature                      |
| `idle.details`      | `string \| function(opts)` | `'Idling'` | Details shown when idle                                       |
| `idle.state`        | `string \| function(opts)` | `nil`      | State shown when idle                                         |
| `idle.tooltip`      | `string \| function(opts)` | `'💤'`      | Tooltip shown when hovering over idle icon                    |
| `idle.icon`         | `string \| function(opts)` | `nil`      | Custom icon URL or asset ID                                   |

## 📝 Text & Assets

| Option    | Type           | Default                           | Description                                   |
| --------- | -------------- | --------------------------------- | --------------------------------------------- |
| `text`    | `table`        | See [Text Options](#text-options) | Customize displayed text for different states |
| `buttons` | `table \| nil` | `nil`                             | Configure [presence buttons](#buttons)        |
| `assets`  | `table \| nil` | `nil`                             | Custom [file icons](#assets) configuration    |

## 🧩 Variables

| Option      | Type    | Default | Description                                                                                                                     |
| ----------- | ------- | ------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `variables` | `table` | `nil`   | Define [custom variables](#custom-variables) for use in string templates. Functions can be used to dynamically generate values. |

## 🪝 Hooks

| Option                      | Type                       | Description                                                                  |
| --------------------------- | -------------------------- | ---------------------------------------------------------------------------- |
| `hooks.on_ready`            | `function`                 | Called when connected to the server and ready for communication with Discord |
| `hooks.on_update`           | `function(opts)`           | Called before building activity                                              |
| `hooks.on_activity`         | `function(opts, activity)` | Called before sending activity                                               |
| `hooks.on_idle`             | `function(opts)`           | Called when idle state changes                                               |
| `hooks.on_workspace_change` | `function(opts)`           | Called when workspace changes                                                |
| `hooks.on_disconnect`       | `function`                 | Called when server disconnects                                               |

## ⚙️ Advanced

| Option                            | Type            | Default     | Description                                                                                                          |
| --------------------------------- | --------------- | ----------- | -------------------------------------------------------------------------------------------------------------------- |
| `advanced.server.pipe_path`       | `string \| nil` | `nil`       | Custom IPC pipe path                                                                                                 |
| `advanced.server.executable_path` | `string \| nil` | `nil`       | Custom server executable path                                                                                        |
| `advanced.server.timeout`         | `number`        | `60000`     | Server shutdown timeout (ms)                                                                                         |
| `advanced.cursor_update_mode`     | `string`        | `'on_move'` | When to update cursor position: `'on_move'`, `'on_hold'`, or `'none'`. See [Cursor Update Mode](#cursor-update-mode) |

---

### Text Options

The `text` table supports both static strings and functions for dynamic content:

```lua
text = {
    editing = function(opts)
        return string.format('Editing %s', opts.filename)
    end,
    viewing = 'Viewing %s',              -- Simple string with filename placeholder
    docs = 'Reading docs',               -- Shown in help buffers
    dashboard = 'Home',                  -- Shown in dashboard buffers
    file_browser = 'Browsing files',     -- Shown in file explorer
    plugin_manager = 'Managing plugins', -- Shown in plugin manager
    lsp_manager = 'Configuring LSP',     -- Shown in LSP manager
    vcs = 'Managing Git',                -- Shown in VCS related filetypes
}
```

### Buttons

Buttons can have static or dynamic labels and URLs:

```lua
buttons = {
    {
        label = function(opts)
            return opts.git_url and 'View Repository' or 'Website'
        end,
        url = function(opts)
            return opts.git_url or 'https://example.com'
        end
    }
}
```

### Assets

Override icons and text for specific filetypes or filenames. Most of the options also support functions.

```lua
assets = {
    ['.rs'] = {
        icon = 'rust',              -- Asset name or URL
        tooltip = 'Rust',           -- Hover text
        text = 'Writing in Rust'    -- Override entire text
    },
    netrw = {
        name = 'Netrw',             -- Override icon name only
        icon = 'default',           -- Use default icon
        type = 'file_browser'       -- Set icon type
    }
}
```

> [!TIP]
> A detailed guide can be found in the [Wiki](https://github.com/vyfor/cord.nvim/wiki/Add-or-change-file-icons).

### Smart Idle

Smart idle ensures that:
- When an instance goes idle, it switches to show the most recent active one
- You're only shown as idle when all instances are actually idle

### Custom Variables

The `variables` option allows you to define custom variables to be used in string templates. These variables can be static values or functions that dynamically generate values based on the current context. By default, the table is populated with the [options table](#options-table) but they can be overridden by user-defined variables.

Example configuration:

```lua
require('cord').setup {
    variables = {
        filename = 'a file',
        problems = function(opts) return #vim.diagnostic.get(0) end,
    },
    text = {
        viewing = 'Viewing ${filename} - ${problems} problems',
    }
}

### User Commands

- `:CordTogglePresence` - Toggle presence visibility
- `:CordShowPresence` - Show presence
- `:CordHidePresence` - Hide presence
- `:CordToggleIdle` - Toggle idle status
- `:CordIdle` - Show idle status
- `:CordUnidle` - Hide idle status
- `:CordClearPresence` - Clear current session's presence
- `:CordClearPresence!` - Clear the rich presence for all sessions

### Cursor Update Mode

The `advanced.cursor_update_mode` option controls how cursor position updates are handled:
- `'on_move'` - Uses `CursorMoved[I]` autocmd, updating on every cursor movement. Most accurate but triggered very often
- `'on_hold'` - Uses `CursorHold[I]` autocmd, updating only after the cursor has been stationary for `'updatetime'` milliseconds. Better performance but less accurate
- `'none'` - Disables cursor position updates entirely

### Options Table

The `opts` parameter passed to all functions and hooks contains the following information:

```lua
{
    manager           = ActivityManager,  -- Reference to the ActivityManager instance

    -- File Information
    filename          = string,           -- Current buffer's filename
    filetype          = string,           -- Current buffer's filetype
    is_read_only      = boolean,          -- Whether the current buffer is read-only

    -- Cursor Information
    cursor_line       = number,           -- Current cursor line number
    cursor_char       = number,           -- Current cursor character number

    -- Timestamp Information
    timestamp         = number,           -- Timestamp passed to the Rich Presence in milliseconds

    -- Workspace Information
    workspace_dir     = string,           -- Current workspace directory
    workspace_name    = string,           -- Current workspace name
    git_url           = string,           -- Current Git repository URL, if any

    -- Editor Information
    is_focused        = boolean,          -- Whether Neovim is focused
    is_idle           = boolean,          -- Whether the session is idle
    buttons           = table,            -- List of configured presence buttons

    -- Asset Information
    type              = string,           -- Which category the asset belongs to, e.g. 'language' or 'docs'
    name              = string,           -- Asset name, if any
    icon              = string,           -- Asset icon URL or name, if any
    tooltip           = string,           -- Hover text for the asset, if any
    text              = string,           -- Custom text to display, if any
}
```

The `ActivityManager` contains useful methods:

### ActivityManager Methods

| Method                       | Description                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `queue_update(force_update)` | Schedules an update to the activity. If `force_update` is true, it bypasses checks and updates immediately.        |
| `pause()`                    | Pauses all events and stops the idle timer.                                                                        |
| `resume()`                   | Resumes events and restarts the idle timer.                                                                        |
| `pause_events()`             | Disables event handling without affecting the idle timer.                                                          |
| `resume_events()`            | Enables event handling and queues an immediate update.                                                             |
| `skip_update()`              | Skips the next update once.                                                                                        |
| `hide()`                     | Pauses events and clears the current activity.                                                                     |
| `toggle()`                   | Toggles between pausing and resuming the activity updates.                                                         |
| `force_idle()`               | Forcibly sets the session to idle.                                                                                 |
| `unforce_idle()`             | Clears the idle state and resumes normal activity.                                                                 |
| `toggle_idle()`              | Toggles between idle and normal activity.                                                                          |
| `set_activity(activity)`     | Sets the rich presence to the provided [activity](#activity-options), offering complete control over the presence. |
| `clear_activity(force)`      | Clears the current activity from the Discord presence. If `force` is true, it completely clears the presence.      |

### Activity Options

| Parameter    | Type     | Description                                                                                          |
| ------------ | -------- | ---------------------------------------------------------------------------------------------------- |
| `type`       | `number` | One of 'playing', 'listening', 'watching'                                                            |
| `state`      | `string` | The user's current state (e.g., "Editing a file").                                                   |
| `details`    | `string` | Detailed information about what the user is doing.                                                   |
| `timestamps` | `object` | Contains `start` and `end` timestamps for the activity.                                              |
| `assets`     | `object` | Defines images and tooltips, including `large_image`, `large_text`, `small_image`, and `small_text`. |
| `buttons`    | `array`  | Array of objects, each with `label` and `url`, defining interactive buttons in the presence.         |
