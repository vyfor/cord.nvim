# 📦 Plugins

Cord comes with several built-in plugins that you can easily enable and configure to enhance your Discord Rich Presence right away. These plugins provide ready-made functionality for common use cases.

## 🚀 Enabling Plugins

To use a built-in plugin, simply add its `require` path to the `plugins` table in your `cord.setup()` configuration.

**Basic Plugin Enablement:**

```lua
require('cord').setup {
  plugins = {
    'cord.plugins.diagnostics', -- Enable the diagnostics plugin
  },
}
```

**Enabling and Configuring a Plugin:**

If a plugin has configuration options, you can provide a configuration table using the plugin's require path as the key in the `plugins` table.

```lua
require('cord').setup {
  plugins = {
    'cord.plugins.diagnostics', -- Enable diagnostics plugin with default settings

    ['cord.plugins.diagnostics'] = { -- Enable AND configure diagnostics plugin
      scope = 'workspace', -- Set scope to 'workspace' instead of default 'buffer'
      severity = vim.diagnostic.severity.WARN, -- Show warnings and above
    },
  },
}
```

## 🔌 Available Plugins

Here's a list of the built-in plugins included with Cord, along with their descriptions and configuration options:

### 🧩 Diagnostics (`cord.plugins.diagnostics`)

**Purpose:**  Adds real-time LSP diagnostics (errors, warnings, hints) information to your Rich Presence. Displays the number of diagnostics in the current buffer or workspace.

**Configuration Options:**

```lua
{
  scope = 'buffer',      -- 'buffer' (default) or 'workspace': Scope of diagnostics to display
  severity = { min = vim.diagnostic.severity.WARN }, -- Diagnostic severity filter (see Neovim `:help diagnostic-severity`)
  override = true,     -- Whether to override default text configurations (recommended: true)
}
```

- **`scope`**:
    - `'buffer'` (default):  Show diagnostics count for the *current buffer* only. Overrides `text.viewing` and `text.editing` to include diagnostics count.
    - `'workspace'`: Show diagnostics count for *all buffers in the workspace*. Overrides `text.workspace` to include diagnostics count.

- **`severity`**:
    -  A table defining the minimum diagnostic severity level to include in the count. Uses Neovim's diagnostic severity levels (see `:help diagnostic-severity`).
    -  Example: `{ min = vim.diagnostic.severity.WARN }` will count warnings and errors, but not information messages or hints.
    -  Defaults to showing errors.

- **`override`**:
    - `true` (default):  The plugin will automatically override the `text.viewing`, `text.editing`, and `text.workspace` options to display diagnostics information. Recommended for seamless integration.
    - `false`: The plugin will *not* override text options. You'll need to manually use the plugin's variables in your `text` configuration to display diagnostics (see "Usage Example" below).

**Variables Added:**

- **`diagnostic`**: A table containing the scoped diagnostics data (useful for advanced customization).
- **`diagnostics`**: A function that returns the number of diagnostics in the current scope (buffer or workspace), based on the plugin's configuration.

**Usage Example (Manual Text Configuration - `override = false`):**

```lua
text = {
  -- In string templates
  editing = 'Editing ${filename} - ${diagnostics} problems',

  -- In functions
  workspace = function(opts) return 'In ' .. opts.workspace .. ' - ' .. opts.diagnostics(opts) .. ' problems' end,
}
```

### 🧩 Local Time (`cord.plugins.local_time`)

**Purpose:**  Sets the Rich Presence timestamp to display the current local clock time (hours, minutes and seconds) instead of elapsed time.

**Configuration Options:**

```lua
{
  affect_idle = true,
}
```

- **`affect_idle`**:
    - `true` (default):  Also sets the timestamp for the idle status to local time.
    - `false`: Only affects the timestamp for active presence, elapsed time will be shown when idle.

**Variables Added:**

- **`local_timestamp`**: A function that returns the zeroed timestamp of the current local time (midnight of the current day).

> [!NOTE]
> Incompatible with any other timestamp-related plugins.

### 🧩 Persistent Timer (`cord.plugins.persistent_timer`)

**Purpose:** Provides a persistent timer that tracks the total time spent across multiple Neovim sessions. This timer is scoped and saved to a data file, so that your time spent on a certain scope continues from where it was left off previously, even after restarting Neovim. It's also able to handle multiple simultaneously open clients without data races.

**Configuration Options:**

```lua
{
  scope = 'workspace', -- 'workspace', 'file', 'filetype', or 'global'
  mode = 'all',        -- 'all', 'active', or 'idle'
  file = vim.fn.stdpath 'data' .. '/cord/plugins/persistent_timer/data.json', -- Path to the timer data file
  save_on = { 'exit', 'focus_change', 'periodic' }, -- Events that trigger a save
  save_interval = 30,  -- Interval in seconds for periodic saves
}
```

- **`scope`**:
  - `'workspace'` (default): Track one continuous timer for the entire workspace.
  - `'file'`: Track a separate timer for each individual file.
  - `'filetype'`: Track a separate timer for each filetype (e.g., all `lua` files share one timer).
  - `'global'`: Track a single timer for all activity within Neovim.
- **`mode`**:
  - `'all'` (default): Count all time the corresponding scope is active.
  - `'active'`: Only accumulate time when you are actively moving or typing.
  - `'idle'`: Only accumulate time when the instance is idle.
- **`file`**:
  - Defines the absolute path to the JSON file where time data is stored.
  - Defaults to a file within Neovim's standard data directory.
- **`save_on`**:
  - A table of strings defining when the timer data should be saved to disk.
  - `'exit'`: Saves when Neovim is closed. **(Recommended)**
  - `'focus_change'`: Saves when you focus away from the Neovim window. Important for multi-client sync.
  - `'periodic'`: Saves automatically at the interval defined by `save_interval`.
- **`save_interval`**:
  - The number of seconds between automatic saves if `'periodic'` is enabled.

> [!NOTE]
> Incompatible with any other timestamp-related plugins.

### 🧩 Scoped Timestamps (`cord.plugins.scoped_timestamps`)

**Purpose:** Tracks elapsed time independently for each buffer or workspace.  Optionally "pauses" and "resumes" the timestamp when switching between buffers, providing more context-aware time tracking.

**Configuration Options:**

```lua
{
  scope = 'buffer', -- 'buffer' (default), 'workspace', or 'idle'
  pause = true,
}
```

- **`scope`**:
    - `'buffer'` (default): Track time elapsed *per buffer*.  Timestamp resets when switching buffers (if `pause = false`), or pauses/resumes (if `pause = true`).
    - `'workspace'`: Track time elapsed *per workspace*. Timestamp is consistent across buffers within the same workspace.
    - `'idle'`: Track time spent in *idle state*. Shows elapsed idle time when in idle, and normal activity time when active.

- **`pause`**:
    - `true` (default): When switching scope, the timestamp "pauses" for the previous scope and "resumes" where it left off when you return to that scope.
    - `false`: When switching scope, the timestamp for the previous scope is not untouched, resulting in two scopes.

**Variables Added:**

- **`get_scoped_timestamp()`**: A function that returns the appropriate timestamp value based on the plugin's `scope` and `pause` settings. This is used internally by the plugin.

> [!NOTE]
> Incompatible with any other timestamp-related plugins.