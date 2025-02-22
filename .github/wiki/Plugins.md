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

    ['cord.plugins.diagnostics'] = { -- Configure diagnostics plugin
      scope = 'workspace', -- Set scope to 'workspace' instead of default 'buffer'
      severity = vim.diagnostic.severity.WARN, -- Show warnings and above
    },
  },
}
```

## 🔌 Available Plugins

Here's a list of the built-in plugins included with Cord, along with their descriptions and configuration options:

### Diagnostics (`cord.plugins.diagnostics`)

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

### Local Time (`cord.plugins.local_time`)

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

>[!NOTE]
> Incompatible with `cord.plugins.local_time`

### Scoped Timestamps (`cord.plugins.scoped_timestamps`)

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

>[!NOTE]
> Incompatible with `cord.plugins.local_time`