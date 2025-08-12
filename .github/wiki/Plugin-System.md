# 🔌 Plugin System

Cord's plugin system allows you to extend its functionality and customize it in modular and reusable ways.

This guide will walk you through creating Cord plugins.

## 🛠️ Plugin Structure: Building Blocks

A Cord plugin is essentially a Lua module that returns a table defining its components. There are two main ways to structure your plugin:

### 1. Simple Table Plugin: For Basic Extensions

For plugins that don't require complex initialization or state management, a simple table is sufficient.  This is great for adding variables, hooks, assets, or default configurations.

```lua
-- plugin.lua
return {
  name = "MyPlugin",
  description = "...",
  variables = { ... },
  hooks = { ... },
  assets = { ... },
  config = { ... }
}
```

### 2. Class-Based Plugin: For Stateful & Configurable Plugins

For more advanced plugins that need to manage internal state, handle user configurations, or perform initialization logic, use a class-based structure. This involves creating a Lua module with a `setup()` function that returns the plugin definition.

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
    description = "...",
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

- [**`variables`**](./Configuration.md#custom-variables):
  An optional table of variables that will be merged with the default options. Do note that overriding existing variables is only allowed for built-in variables.

- [**`hooks`**](./Configuration.md#-hooks):
  An optional table of hooks that will be added to the existing list of hooks. Hooks can optionally define a priority that will be used to determine the order of execution. User-defined hooks are prioritized by default.

- [**`assets`**](./Configuration.md#assets):
  An optional table of assets that will be merged with the default assets.

- [**`config`**](./Configuration.md#default-config):
  An optional table of configuration options that will be merged with other plugins' configuration options, and then with the user's configuration which takes precedence.

## 🎯 Best Practices

- Use short, descriptive names for plugins
- Provide a description of what the plugin does
- Do not override critical parts of Cord's configuration; only provide what's necessary
- In plugin initialization, use `error` to raise errors if user configuration is invalid
- In plugin runtime, use `require('cord.plugin.log')` to log messages and errors
- Use function-based configuration over string templates wherever possible. You can still provide custom variables as they are only evaluated when needed

> Example plugin: [`diagnostics.lua`](https://github.com/vyfor/cord.nvim/blob/master/lua/cord/plugins/diagnostics.lua)