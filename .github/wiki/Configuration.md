# ⚡ Configuration

A comprehensive guide to configuring the Cord plugin to your liking. All options are set through the `setup()` function:

```lua
require('cord').setup {
    -- Your configuration options go here
}
```

If you prefer to start with a clean slate, you can copy the [Default Configuration](#default-configuration) below and modify it to your liking. Remember, you only need to specify the options you want to change; any unspecified options will use their default values.

Here's the complete default configuration for Cord. You can use this as a starting point for your own setup.

<details id="default-config">
  <summary>Expand to view default configuration</summary>

```lua
{
  enabled = true,
  log_level = vim.log.levels.OFF,
  editor = {
    client = 'neovim',
    tooltip = 'The Superior Text Editor',
    icon = nil,
  },
  display = {
    theme = 'onyx',
    swap_fields = false,
    swap_icons = false,
  },
  timestamp = {
    enabled = true,
    reset_on_idle = false,
    reset_on_change = false,
  },
  idle = {
    enabled = true,
    timeout = 300000,
    show_status = true,
    ignore_focus = true,
    unidle_on_focus = true,
    smart_idle = true,
    details = 'Idling',
    state = nil,
    tooltip = '💤',
    icon = nil,
  },
  text = {
    workspace = function(opts) return 'In ' .. opts.workspace end,
    viewing = function(opts) return 'Viewing ' .. opts.filename end,
    editing = function(opts) return 'Editing ' .. opts.filename end,
    file_browser = function(opts) return 'Browsing files in ' .. opts.name end,
    plugin_manager = function(opts) return 'Managing plugins in ' .. opts.name end,
    lsp = function(opts) return 'Configuring LSP in ' .. opts.name end,
    docs = function(opts) return 'Reading ' .. opts.name end,
    vcs = function(opts) return 'Committing changes in ' .. opts.name end,
    notes = function(opts) return 'Taking notes in ' .. opts.name end,
    debug = function(opts) return 'Debugging in ' .. opts.name end,
    test = function(opts) return 'Testing in ' .. opts.name end,
    diagnostics = function(opts) return 'Fixing problems in ' .. opts.name end,
    games = function(opts) return 'Playing ' .. opts.name end,
    terminal = function(opts) return 'Running commands in ' .. opts.name end,
    dashboard = 'Home',
  }, 
  buttons = nil,
  -- buttons = {
  --   {
  --     label = 'View Repository',
  --     url = function(opts) return opts.repo_url end,
  --   },
  -- },
  assets = nil,
  variables = nil,
  hooks = {
    ready = nil,
    shutdown = nil,
    pre_activity = nil,
    post_activity = nil,
    idle_enter = nil,
    idle_leave = nil,
    workspace_change = nil,
  },
  plugins = nil,
  advanced = {
    plugin = {
      autocmds = true,
      cursor_update = 'on_hold',
      match_in_mappings = true,
    },
    server = {
      update = 'fetch',
      pipe_path = nil,
      executable_path = nil,
      timeout = 300000,
    },
    discord = {
      reconnect = {
        enabled = false,
        interval = 5000,
        initial = true,
      },
    },
  },
}
```
</details>

## 🎨 Editor

| Option           | Type            | Default                      | Description                                                                                                                             |
| ---------------- | --------------- | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `editor.client`  | `string`        | `'neovim'`                   | Client identifier. Can be `'vim'`, `'neovim'`, `'lunarvim'`, `'nvchad'`, `'astronvim'`, `'lazyvim'`, or a custom Discord application ID |
| `editor.tooltip` | `string`        | `'The Superior Text Editor'` | Tooltip shown when hovering over editor icon                                                                                            |
| `editor.icon`    | `string \| nil` | `nil`                        | Custom icon URL or asset ID when using custom client ID                                                                                 |

## 📊 Display

| Option                | Type      | Default | Description                                                                     |
| --------------------- | --------- | ------- | ------------------------------------------------------------------------------- |
| `display.theme`       | `string`  | `onyx`  | Choose between different icon themes; one of 'onyx' (dark) or 'pastel' (accent) |
| `display.swap_fields` | `boolean` | `false` | Show workspace name before filename                                             |
| `display.swap_icons`  | `boolean` | `false` | Use editor icon as large image                                                  |

## ⏰ Timestamp

| Option                      | Type      | Default | Description                              |
| --------------------------- | --------- | ------- | ---------------------------------------- |
| `timestamp.enabled`         | `boolean` | `true`  | Show elapsed time in presence            |
| `timestamp.reset_on_idle`   | `boolean` | `false` | Reset timestamp when entering idle state |
| `timestamp.reset_on_change` | `boolean` | `false` | Reset timestamp when presence changes    |

## 💤 Idle

| Option                 | Type                       | Default                                                                               | Description                                                   |
| ---------------------- | -------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| `idle.enabled`         | `boolean`                  | `true`                                                                                | Enable idle status detection                                  |
| `idle.timeout`         | `number`                   | `300000`                                                                              | Milliseconds before marking the session as idle               |
| `idle.show_status`     | `boolean`                  | `true`                                                                                | Show idle status in presence, or hide the presence if `false` |
| `idle.ignore_focus`    | `boolean`                  | `true`                                                                                | Show idle despite Neovim having focus                         |
| `idle.unidle_on_focus` | `boolean`                  | `true`                                                                                | Unidle the session when Neovim gains focus                    |
| `idle.smart_idle`      | `boolean`                  | `true`                                                                                | Enable [smart idle](#smart-idle) feature                      |
| `idle.details`         | `string \| function(opts)` | `'Idling'`                                                                            | Details shown when idle                                       |
| `idle.state`           | `string \| function(opts)` | `nil`                                                                                 | State shown when idle                                         |
| `idle.tooltip`         | `string \| function(opts)` | `'💤'`                                                                                 | Tooltip shown when hovering over idle icon                    |
| `idle.icon`            | `string \| function(opts)` | [`default idle icon`](https://github.com/vyfor/icons/blob/master/icons/onyx/idle.png) | Custom icon URL or asset ID                                   |

## 📝 Text

| Option           | Type                                  | Default                        | Description                                        |
| ---------------- | ------------------------------------- | ------------------------------ | -------------------------------------------------- |
| `workspace`      | `string \| function(opts) \| boolean` | `In {workspace}`               | Text shown when in a workspace                     |
| `viewing`        | `string \| function(opts) \| boolean` | `Viewing {filename}`           | Text shown when viewing a file                     |
| `editing`        | `string \| function(opts) \| boolean` | `Editing {filename}`           | Text shown when editing a file                     |
| `file_browser`   | `string \| function(opts) \| boolean` | `Browsing files in {name}`     | Text shown when in a file browser                  |
| `plugin_manager` | `string \| function(opts) \| boolean` | `Managing plugins in {name}`   | Text shown when in a plugin manager                |
| `lsp`            | `string \| function(opts) \| boolean` | `Configuring LSP in {name}`    | Text shown when in an LSP manager                  |
| `docs`           | `string \| function(opts) \| boolean` | `Reading {name}`               | Text shown when in a docs buffer                   |
| `vcs`            | `string \| function(opts) \| boolean` | `Committing changes in {name}` | Text shown when in a VCS buffer                    |
| `notes`          | `string \| function(opts) \| boolean` | `Taking notes in {name}`       | Text shown when in a notes buffer                  |
| `debug`          | `string \| function(opts) \| boolean` | `Debugging in {name}`          | Text shown when in a debug-related plugin buffer   |
| `test`           | `string \| function(opts) \| boolean` | `Testing in {name}`            | Text shown when in a testing-related plugin buffer |
| `diagnostics`    | `string \| function(opts) \| boolean` | `Fixing problems in {name}`    | Text shown when in a diagnostics buffer            |
| `games`          | `string \| function(opts) \| boolean` | `Playing {name}`               | Text shown when in a game buffer                   |
| `terminal`       | `string \| function(opts) \| boolean` | `Running commands in {name}`   | Text shown when in a terminal buffer               |
| `dashboard`      | `string \| function(opts) \| boolean` | `'Home'`                       | `Home`                                             | Text shown when in a dashboard buffer |

> Also see [Text Options](#text-options)

## 🧩 Variables

| Option      | Type                      | Default | Description                                                                                                                                                                                                                                                               |
| ----------- | ------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `variables` | `table \| boolean \| nil` | `nil`   | Define [custom variables](#custom-variables) for use in string templates. Functions can be used to dynamically generate values. If `true`, uses the default [options table](#options-table), if `table`, extends the default table, if `false`, disables custom variables |

## 🪝 Hooks

| Option                   | Type                                                                                 | Description                                                                                                        |
| ------------------------ | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `hooks.ready`            | `function(manager) \| table<fun: function(manager), priority: number>`               | Called when connected to the server and ready for communication with Discord ([manager](#activitymanager-methods)) |
| `hooks.shutdown`         | `function() \| table<fun: function(), priority: number>`                             | Called when connection to Discord is closed                                                                        |
| `hooks.pre_activity`     | `function(opts) \| table<fun: function(opts), priority: number>`                     | Called before building activity ([opts](#options-table))                                                           |
| `hooks.post_activity`    | `function(opts, activity) \| table<fun: function(opts, activity), priority: number>` | Called after building activity, but before sending it ([opts](#options-table), [activity](#activity))              |
| `hooks.idle_enter`       | `function(opts) \| table<fun: function(opts), priority: number>`                     | Called when entering idle state ([opts](#options-table))                                                           |
| `hooks.idle_leave`       | `function(opts) \| table<fun: function(opts), priority: number>`                     | Called when leaving idle state ([opts](#options-table))                                                            |
| `hooks.workspace_change` | `function(opts) \| table<fun: function(opts), priority: number>`                     | Called when workspace changes ([opts](#options-table))                                                             |

## 🔌 Plugins

| Option    | Type                               | Description                                                          |
| --------- | ---------------------------------- | -------------------------------------------------------------------- |
| `plugins` | `string[] \| table<string, table>` | Extend Cord with plugins. See the [Wiki](./Plugins.md) for more info |

> If you want to develop your own plugin, check out Cord's [Plugin System](./Plugin-System.md)

## ⚙️ Advanced

| Option                                | Type            | Default     | Description                                                                                                                                                                                        |
| ------------------------------------- | --------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `advanced.plugin.autocmds`            | `boolean`       | `true`      | Enable autocmds                                                                                                                                                                                    |
| `advanced.plugin.cursor_update`       | `string`        | `'on_hold'` | When to update cursor position: `'on_move'`, `'on_hold'`, or `'none'`. See [Cursor Update Mode](#cursor-update-mode)                                                                               |
| `advanced.plugin.match_in_mappings`   | `boolean`       | `true`      | Whether to match against file extensions in mappings                                                                                                                                               |
| `advanced.server.update`              | `string`        | `'fetch'`   | Default way to acquire the server executable either if the executable is not found or a manual update is requested: `'fetch'` - fetch from GitHub, `'build'` - build from source, `'none'` - no-op |
| `advanced.server.pipe_path`           | `string \| nil` | `nil`       | Custom IPC pipe path                                                                                                                                                                               |
| `advanced.server.executable_path`     | `string \| nil` | `nil`       | Custom server executable path                                                                                                                                                                      |
| `advanced.server.timeout`             | `number`        | `300000`    | Server shutdown timeout (ms)                                                                                                                                                                       |
| `advanced.discord.reconnect.enabled`  | `boolean`       | `false`     | Whether reconnection is enabled. Has minimal impact on performance                                                                                                                                 |
| `advanced.discord.reconnect.interval` | `number`        | `5000`      | Reconnection interval in milliseconds, 0 to disable                                                                                                                                                |
| `advanced.discord.reconnect.initial`  | `boolean`       | `true`      | Whether to reconnect if initial connection fails                                                                                                                                                   |

---

## Text Options

The `text` table allows you to customize the displayed text for different states. You can customize it in three different ways:

1. Using simple strings:
```lua
text = {
    editing = 'Editing a file',
    viewing = 'Viewing a file',
}
```

2. Using functions for dynamic text:
```lua
text = {
    editing = function(opts)
        return string.format('Editing %s', opts.filename)
    end,
}
```

3. Using string templates (requires enabling variables):
```lua
{
    text = {
        editing = 'Editing ${filename}',
        file_browser = 'Browsing files in ${tooltip}',
    },
    variables = true, -- Enable string templates
}
```

> To see all available options, refer to the [default configuration](#default-config).

It is also possible to use boolean values to completely disable a category:

```lua
text = {
    workspace = '',         -- Omit the text from the activity, meaning it will only have one row of text
    games = function() end, -- Returning `nil` is the same as above

    file_browser = true,    -- Ignore these types of buffers and the current activity will remain unchanged
    plugin_manager = false, -- Hide the activity for these types of buffers

    -- Also applicable to functions
    diagnostics = function(opts)
        -- Only show diagnostics activity if there are problems, otherwise do nothing
        return #vim.diagnostics.get(vim.api.nvim_get_current_buf()) > 0 and 'Fixing problems in ' .. opts.tooltip or true
    end,
}
```


## Buttons

Buttons can have static or dynamic labels and URLs:

```lua
buttons = {
    {
        label = function(opts)
            return opts.repo_url and 'View Repository' or 'Website'
        end,
        url = function(opts)
            return opts.repo_url or 'https://example.com'
        end
    }
}
```

## Assets

Override icons and text for specific filetypes or filenames. Most of the options also support functions.

```lua
assets = {
    ['.rs'] = {
        icon = 'rust',              -- Asset name or URL
        tooltip = 'Rust',           -- Hover text
        text = 'Writing in Rust'    -- Override entire text
    },
    netrw = {
        name = 'Netrw',             -- Override asset name only
        type = 'file_browser'       -- Set asset type
    }
}
```

> [!TIP]
> A detailed guide can be found in the [Wiki](./Assets.md).

Some languages cannot be identified solely by their filename or extension. In such cases, we can utilize the `vim.filetype.add` function to add extra patterns for filetype detection:

```lua
vim.filetype.add {
  pattern = {
    ['.*/waybar/config'] = 'jsonc',
    ['.*/hypr/.*%.conf'] = 'hyprlang',
    -- ...
  },
}
```

## Smart Idle

Smart idle ensures that:
- When an instance goes idle, it switches to show the most recent active one
- You're only shown as idle when all instances are actually idle

## Custom Variables

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
```

## User Commands

>[!NOTE]
> Autocompletion is fully supported.
> To see the list of all available features, hit `<Tab>` after typing `:Cord enable`

- `:Cord enable` - Show presence
  - `:Cord enable <feature>` - Enable a feature
- `:Cord disable` - Hide presence
  - `:Cord disable <feature>` - Disable a feature
- `:Cord toggle` - Toggle presence
  - `:Cord toggle <feature>` - Toggle a feature
- `:Cord presence` - Toggle presence display
  - `:Cord presence show` - Show presence
  - `:Cord presence hide` - Hide presence
  - `:Cord presence suppress` - Suppress presence for current Neovim session
  - `:Cord presence toggle` - Toggle presence
  - `:Cord presence toggle_suppress` - Toggle presence suppression
- `:Cord idle` - Toggle idle state
  - `:Cord idle toggle` - Toggle idle state
  - `:Cord idle show` - Show idle state
  - `:Cord idle hide` - Hide idle state
  - `:Cord idle force` - Force idle state
- `:Cord update` - Update the server executable using the configured update mode (fetch by default)
  - `:Cord update check` - Check for server updates
  - `:Cord update fetch` - Fetch the server executable from GitHub using `curl`
  - `:Cord update build` - Build the server executable using `cargo`
- `:Cord status` - Show connection status
- `:Cord version` - Show current server version
- `:Cord restart` - Restart the server
- `:Cord shutdown` - Disconnect from Discord and shutdown the server
- `:Cord health` - Validate user configuration

## Cursor Update Mode

The `advanced.cursor_update_mode` option controls how cursor position updates are handled:
- `'on_move'` - Uses `CursorMoved[I]` autocmd, updating on every cursor movement. Most accurate but triggered very often
- `'on_hold'` - Uses `CursorHold[I]` autocmd, updating only after the cursor has been stationary for `'updatetime'` milliseconds. Better performance but less accurate
- `'none'` - Disables cursor position updates entirely

## Options Table

The `opts` parameter passed to all functions and hooks contains the following information:

```lua
{
  manager           = ActivityManager,  -- Reference to the ActivityManager instance

  -- File Information
  name              = string?,          -- Name associated with the current mapping, if any
  tooltip           = string?,          -- Tooltip associated with the current mapping, if any
  filename          = string,           -- Current buffer's filename
  filetype          = string,           -- Current buffer's filetype
  buftype           = string,           -- Current buffer's buftype
  is_read_only      = boolean,          -- Whether the current buffer is read-only

  -- Cursor Information
  cursor_line       = number,           -- Current cursor line number
  cursor_char       = number,           -- Current cursor character number

  -- Timestamp Information
  timestamp         = number,           -- Timestamp passed to the Rich Presence in milliseconds

  -- Workspace Information
  workspace         = string?,          -- Current workspace name
  workspace_dir     = string?,          -- Current workspace directory
  repo_url          = string?,          -- Current Git repository URL, if any

  -- Editor Information
  is_focused        = boolean,          -- Whether Neovim is focused
  is_idle           = boolean,          -- Whether the session is idle
  buttons           = table?,           -- List of configured presence buttons

  -- Asset Information
  type              = string,           -- Which category the asset belongs to, e.g. 'language' or 'docs'
  icon              = string?,          -- Asset icon URL or name, if any
  text              = string?,          -- Custom text to display, if any
}
```

The `ActivityManager` contains useful methods:

## ActivityManager Methods

| Method                               | Description                                                                                                        |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| `manager:queue_update(force_update)` | Schedules an update to the activity. If `force_update` is true, it bypasses checks and updates immediately.        |
| `manager:pause()`                    | Pauses all events and stops the idle timer.                                                                        |
| `manager:resume()`                   | Resumes events and restarts the idle timer.                                                                        |
| `manager:pause_events()`             | Disables event handling without affecting the idle timer.                                                          |
| `manager:resume_events()`            | Enables event handling and queues an immediate update.                                                             |
| `manager:skip_update()`              | Skips the next update once.                                                                                        |
| `manager:hide()`                     | Pauses events and clears rich presence.                                                                            |
| `manager:suppress()`                 | Pauses events and suppresses sending presence updates for the current Neovim session.                              |
| `manager:toggle()`                   | Toggles between pausing and resuming the activity updates.                                                         |
| `manager:toggle_suppress()`          | Toggles between suppressing and resuming sending presence updates for the current Neovim session.                  |
| `manager:idle()`                     | Sets the session to idle.                                                                                          |
| `manager:force_idle()`               | Forcibly sets the session to idle until unforced.                                                                  |
| `manager:unidle()`                   | Clears the idle state and resumes normal activity.                                                                 |
| `manager:toggle_idle()`              | Toggles between idle and normal activity.                                                                          |
| `manager:set_activity(activity)`     | Sets the rich presence to the provided [activity](#activity-options), offering complete control over the presence. |
| `manager:clear_activity(force)`      | Clears the current activity from the server. If `force` is true, it completely clears the presence.                |

## Activity Options

| Parameter    | Type      | Description                                                                                          |
| ------------ | --------- | ---------------------------------------------------------------------------------------------------- |
| `type`       | `number`  | One of 'playing', 'listening', 'watching'                                                            |
| `state`      | `string`  | The user's current state (e.g., "Editing a file").                                                   |
| `details`    | `string`  | Detailed information about what the user is doing.                                                   |
| `timestamps` | `table`   | Contains `start` and `end` timestamps for the activity.                                              |
| `assets`     | `table`   | Defines images and tooltips, including `large_image`, `large_text`, `small_image`, and `small_text`. |
| `buttons`    | `array`   | Array of objects, each with `label` and `url`, defining interactive buttons in the presence.         |
| `is_idle`    | `boolean` | Whether the activity should be considered as idle.                                                   |

## Useful Functions

- `require('cord.api.icon').get(name: string, theme?: string): string`
  - **Get Icon URL:** Returns the URL for the specified icon `name` and optional `theme`. If `theme` is not provided, it defaults to the configured `display.theme`. Useful for referencing Cord's built-in icons in your custom configurations.
