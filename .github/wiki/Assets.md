# 📂 Custom Assets

Cord allows you to go beyond the default icons and fully customize the assets used in your Discord Rich Presence. This guide will walk you through how to define and configure custom assets to perfectly match your workflow and style.

## 🛠️ Structure

Custom assets are configured within the `assets` table in your `cord.setup()` call. The basic structure is as follows:

```lua
require('cord').setup {
  assets = {
    -- "key" = "icon" or { asset_options }
  },
}
```

Here, `"key"` is the identifier for the asset, and `"value"` can be either a simple string for the icon name or a table for more detailed asset options.

## 🔑 Asset Keys: Identifying What to Customize

The `key` in your asset configuration determines what type of file or context the custom asset will apply to. You can use the following key types:

- **Filetype**: Use a Vim filetype string (e.g., `"lua"`, `"python"`) to set assets for specific languages.
- **Filename**: Use a specific filename (e.g., `"Cargo.toml"`, `"README.md"`) to customize assets for particular files.
- **File Extension**: Use a file extension (e.g., `".rs"`, `".js"`) to apply assets to all files with that extension.
- **Special Keys**: Cord provides special keys for generic scenarios (see [Overriding Defaults](#overriding-defaults)).

## ⚙️ Asset Options: Fine-Tuning Your Icons

When you use a table as the `value` in your asset configuration, you can specify these options to fine-tune your custom asset:

| Option    | Type                  | Description                                                                                                                |
| --------- | --------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `icon`    | `string \| function ` | **Required.** A direct URL to the icon image or the name of a rich presence asset (if using your own Discord application). |
| `tooltip` | `string \| function`  | Text that appears when you hover over the icon in Discord.                                                                 |
| `name`    | `string \| function`  | An optional override for the icon's name. Generally used for types other than `language`.                                  |
| `text`    | `string \| function`  | An optional override for the icon's text. **Completely replaces** the default text.                                        |
| `type`    | `string`              | Specifies the context of the asset.  See [Asset Types](#asset-types) for available options.                                |

### Asset Types

The `type` option categorizes your asset, influencing how Cord displays it. Available types include:
- `language` (default)
- `docs`
- `plugin_manager`
- `lsp_manager`
- `file_browser`
- `vcs`
- `workspace`
- `dashboard`
- `notes`
- `debug`
- `test`
- `diagnostics`
- `games`
- `terminal`

## 🚀 Examples

Let's look at some practical examples of custom asset configuration:

**1. Simple Icon Replacement by Extension:**

```lua
assets = {
  ['.rs'] = 'rust_icon', -- Use the "rust_icon" for all Rust files
}
```

This will use the icon named `rust_icon` (which should be defined in your Discord Developer Portal if you are using a custom application, or a URL) for any file with the `.rs` extension.

**2. Custom Icon and Tooltip for a Filetype:**

```lua
assets = {
  lua = {
    icon = 'lua_custom_icon',
    tooltip = 'Lua Script', -- Custom tooltip for Lua files
  },
}
```

This example sets a custom icon and tooltip specifically for Lua files (the `lua` filetype).

**3. Using Cord's Built-in Icons:**

```lua
assets = {
  ['.settings'] = require('cord.api.icon').get('gear'),
}
```

This makes use of Cord's default icon library, using the "gear" icon for files with the `.settings` extension.

**4. Plugin Manager Asset with Custom Text:**

```lua
assets = {
  lazy = {
    name = 'Lazy',
    icon = 'https://...',
    tooltip = 'lazy.nvim Plugin Manager',
    type = 'plugin_manager',
    text = function(opts) -- Dynamic text
      return 'Managing plugins in ' .. opts.name
    end,
  },
}
```

Here, we define a custom asset for the `lazy.nvim` plugin manager, setting its name, icon, tooltip, type, and dynamic text.

**5. Overriding Tooltip and Text, Keeping Default Icon:**

```lua
assets = {
  lua = {
    tooltip = '.lua file',     -- Just change the tooltip
    text = 'Writing Lua Code', -- Override the text
  },
}
```

This example only modifies the tooltip and text for Lua files, keeping Cord's default icon.

## 🧰 Overriding Defaults

Cord provides special keys to define assets for generic situations:

- `['Cord.new']`: Asset for a new, empty buffer when Neovim starts without file arguments, when both filetype and filename are empty.
- `['Cord.unknown']`: Asset when the filetype is not detected/defined in Cord.
- `['Cord.override']`: This key overrides *all* existing icons, applying the specified asset everywhere.

**Example of Default Overrides:**

```lua
assets = {
  ['Cord.new'] = 'default_file_icon',
  ['Cord.unknown'] = 'question_mark_icon',
  ['Cord.override'] = 'my_brand_icon',
}
```

## 💡 Tip

Sometimes, filetypes aren't automatically recognized by Neovim based on filename or extension alone. Use `vim.filetype.add` to help the editor identify them:

```lua
vim.filetype.add {
  pattern = {
    ['.*/waybar/config'] = 'jsonc',    -- Recognize waybar config files as jsonc
    ['.*/hypr/.*%.conf'] = 'hyprlang', -- Hyprland configs as hyprlang
    -- ... add more custom patterns
  },
}
```