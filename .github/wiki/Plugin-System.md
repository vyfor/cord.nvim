# 🔌 Plugin System

## 📖 Introduction
The Cord plugin system provides a simple way to extend and customize your Discord presence without complex manual configuration. It allows:
- Providing common presence configurations out of the box
- Sharing presence configurations among the community
- Avoiding the hassle of manually configuring Cord for basic features

## 🛠️ Plugin Definition

Plugins must return a table with at least a `name` field.

### Simple Table
For basic plugins that don't need initialization:
```lua
-- plugin.lua
return {
  name = "MyPlugin",
  variables = { ... },
  hooks = { ... },
  assets = { ... },
  config = { ... }
}
```

### Class
For more complex plugins that need initialization:
```lua
-- plugin_with_config.lua
local M = {
    -- Options internal to the plugin can be defined and stored here or at the top-level
    state = '',
    config = {},
}

M.setup = function(config)
  -- Initialize plugin
  M.config = vim.tbl_deep_extend('force', M.config, config)

  return {
    name = "MyPlugin",
    variables = { ... },
    hooks = { ... },
    assets = { ... },
    config = { ... }
  }
end

return M
```

## ⚙️ Options

- **`name`**:
  The name of the plugin. It must be unique across all plugins.

- **`description`**:
  An optional description of the plugin.

- [**`variables`**](https://github.com/vyfor/cord.nvim/wiki/Configuration#custom-variables):
  An optional table of variables that will be merged with the default options. Do note that overriding existing variables is only allowed for built-in variables.

- [**`hooks`**](https://github.com/vyfor/cord.nvim/wiki/Configuration#-hooks):
  An optional table of hooks that will be added to the existing list of hooks. Hooks can optionally define a priority that will be used to determine the order of execution. User-defined hooks are prioritized by default.

- [**`assets`**](https://github.com/vyfor/cord.nvim/wiki/Configuration#assets):
  An optional table of assets that will be merged with the default assets.

- [**`config`**](https://github.com/vyfor/cord.nvim/wiki/Configuration#default-config):
  An optional table of configuration options that will be merged with other plugins' configuration options, and then with the user's configuration which takes precedence.

## 🎯 Best Practices

- Use short, descriptive names for plugins
- Provide a description of what the plugin does
- Do not override critical parts of Cord's configuration; only provide what's necessary
- In plugin initialization, use `error` to raise errors if user configuration is invalid
- In plugin runtime, use `require('cord.plugin.log')` to log messages and errors
- Use function-based configuration over string templates wherever possible. You can still provide custom variables as they are only evaluated when needed

> Example plugin: [`diagnostics.lua`](https://github.com/vyfor/cord.nvim/blob/client-server/lua/cord/plugins/diagnostics.lua)