## Configuration Structure

Cord allows the customization of the appearance of a file icon. To do so, you need to modify the `assets` table in the setup configuration of the plugin:

```lua
require('cord').setup {
  assets = {
    -- key: string = value: string or table
  },
}
```

## Icon Configuration Options

| Option      | Type              | Description                                                                                                  |
| ----------- | ----------------- | ------------------------------------------------------------------------------------------------------------ |
| **(key)**   | `string`          | The key can be a Vim filetype (like `lua`), a filename (like `Cargo.toml`), or a file extension (like `.rs`) |
| **(value)** | `string \| table` | This can either be a string pointing to the icon or a table with options below                               |
| `icon`      | `string`          | A direct URL to the icon image or the name of the rich presence asset (in case you use your own application) |
| `tooltip`   | `string`          | Text that appears when hovering over the icon                                                                |
| `name`      | `string`          | An optional override for the icon name. Redundant for `language` types                                       |
| `text`      | `string`          | An optional override for the icon text. Unlike other options, this will fully override the text              |
| `type`      | `string`          | Specifies the context of the asset.                                                                          |

Available types:
- `language` (default)
- `docs`
- `plugin_manager`
- `lsp_manager`
- `file_browser`
- `vcs`
- `workspace`
- `dashboard`
- `notes`

**Examples**:
```lua
['.rs'] = 'rust_icon'
```
```lua
lazy = {
  name = 'Lazy', -- `config.text.plugin_manager + name` = "Managing plugins in Lazy"
  icon = 'https://example.com/lazy.png',
  tooltip = 'lazy.nvim',
  type = 'plugin_manager'
}
```
> Use Cord's existing icon
```lua
['.settings'] = require('cord.api.icon').get('gear') -- or { icon = require(...) }
```
> In the below configuration, only the tooltip for Lua files is changed, while the icon and name remain as provided by Cord's defaults.
```lua
lua = {
  -- Overrides default tooltip and text only
  tooltip = '.lua file',
  text = 'Writing in Lua' -- `config.text.editing` is ignored
}
```

## Overriding Default Icons

In addition to setting icons for specific filetypes or extensions, you can also define icons for generic scenarios:

- `['Cord.new']`: Sets the icon for a new buffer when Neovim is opened without any file arguments.
- `['Cord.unknown']`: Used when the filetype is not detected by the editor or not supported in Cord at the moment.
- `['Cord.override']`: Overrides all of the existing icons.

**Example**:
```lua
['Cord.new'] = 'default_icon'
['Cord.unknown'] = 'unknown_icon'
['Cord.override'] = 'rust' -- Rust... Rust everywhere...
```