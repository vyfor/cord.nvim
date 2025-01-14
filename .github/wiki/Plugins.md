# 📦 Built-in Plugins

Cord comes with several built-in plugins that can be configured in your setup. To use a plugin, simply add its `require` path to the `plugins` table in your cord setup:

```lua
require('cord').setup {
  plugins = {
    'cord.plugins.diagnostics',

    -- optionally, provide a custom configuration
    {
      'cord.plugins.diagnostics',
      config = {
        severity = vim.diagnostic.severity.ERROR,
      }
    }
  }
}
```

## Available Plugins

### Diagnostics (`cord.plugins.diagnostics`)
Adds diagnostic information.

#### Configuration

```lua
{
  -- 'buffer' or 'workspace'
  scope = 'buffer',
  -- https://neovim.io/doc/user/diagnostic.html#diagnostic-quickstart
  severity = { min = vim.diagnostic.severity.WARN },
  -- whether to override the default configuration if no user configuration is provided
  override = true,
}
```

- On `buffer` scope, diagnostics are only shown for the current buffer. `text.viewing` and `text.editing` are overriden to display the diagnostics count.
- On `workspace` scope, diagnostics are shown for all buffers. `text.workspace` is overriden to display the diagnostics count.

It will also add the following variables:
  - `diagnostic` - a table containing the scoped diagnostics
  - `diagnostics` - the number of diagnostics in the current buffer or workspace
  
#### Usage example (Optional)
```lua
text = {
  -- In string templates
  editing = 'Editing ${filename} - ${diagnostics} problems',

  -- In functions
  workspace = function(opts) return 'In ' .. opts.workspace_name .. ' - ' .. opts.diagnostics(opts) .. ' problems' end,
}
```

### Local Time (`cord.plugins.local_time`)
Displays the local clock time as the start timestamp.

#### Configuration
```lua
{
  -- whether to override the timestamp for the idle status
  affect_idle = true,
}
```

It will also add the following variables:
  - `local_timestamp` - the zeroed timestamp of the current local time