# 🔌 Extension System

Cord's extension system allows you to extend its functionality and customize it in modular and reusable ways.

This guide will walk you through creating Cord extensions.

## 🛠️ Extension Structure: Building Blocks

A Cord extension is essentially a Lua module that returns a table defining its components. There are two main ways to structure your extension:

### 1. Simple Table Extension: For Basic Extensions

For extensions that don't require complex initialization or state management, a simple table is sufficient.  This is great for adding variables, hooks, assets, or default configurations.

```lua
-- extension.lua
return {
  name = "MyExtension",
  description = "...",
  variables = { ... },
  hooks = { ... },
  assets = { ... },
  config = { ... }
}
```

### 2. Class-Based Extension: For Stateful & Configurable Extensions

For more advanced extensions that need to manage internal state, handle user configurations, or perform initialization logic, use a class-based structure. This involves creating a Lua module with a `setup()` function that returns the extension definition.

```lua
-- extension_with_config.lua
local M = {
    -- Options internal to the extension can be defined and stored here or at the top-level
    state = '',
    config = {},
}

M.setup = function(config)
  -- Initialize extension
  M.config = vim.tbl_deep_extend('force', M.config, config)

  return {
    name = "MyExtension",
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
  The name of the extension. It must be unique across all extensions.

- **`description`**:
  An optional description of the extension.

- [**`variables`**](./Configuration.md#custom-variables):
  An optional table of variables that will be merged with the default options. Do note that overriding existing variables is only allowed for built-in variables.

- [**`hooks`**](./Configuration.md#-hooks):
  An optional table of hooks that will be added to the existing list of hooks. Hooks can optionally define a priority that will be used to determine the order of execution. User-defined hooks are prioritized by default.

- [**`assets`**](./Configuration.md#assets):
  An optional table of assets that will be merged with the default assets.

- [**`config`**](./Configuration.md#default-config):
  An optional table of configuration options that will be merged with other extensions' configuration options, and then with the user's configuration which takes precedence.

## 🎯 Best Practices

- Use short, descriptive names for extensions
- Provide a description of what the extension does
- Do not override critical parts of Cord's configuration; only provide what's necessary
- In extension initialization, use `error` to raise errors if user configuration is invalid
- In extension runtime, use `require('cord.api.log')` to log messages and errors
- Use function-based configuration over string templates wherever possible. You can still provide custom variables as they are only evaluated when needed

> Example extension: [`diagnostics.lua`](https://github.com/vyfor/cord.nvim/blob/master/lua/cord/extensions/diagnostics.lua)