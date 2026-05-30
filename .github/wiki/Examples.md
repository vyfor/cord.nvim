# 🎨 Examples

Learn how to customize your Discord presence with examples. The possibilities are endless, and the only limit is your creativity!

## 🚀 Getting Started

### Enabling the repository button

```lua
require('cord').setup {
  buttons = {
    {
      label = 'View Repository',
      url = function(opts)
        return opts.repo_url -- only show the button if a repo URL is found
      end,
    },
  },
}
```

### Using a custom name for your presence

The text next to "Playing" (e.g. *"Neovim"*) comes from the Discord
application Cord connects to. To show a custom name, create an application in
the [Discord Developer Portal](https://discord.com/developers/applications),
then pass its application ID as `editor.client`:

```lua
require('cord').setup {
  editor = {
    client = '1234567890123456789',
    -- icon = 'https://...', -- optional icon override
  },
}
```

>[!NOTE]
> Changing `editor.client` takes effect after `:Cord restart`.

---

## 🎨 Appearance

Cord provides 120+ icons across several themes, each with `dark`, `light`, and
`accent` flavors. See [Themes](https://github.com/vyfor/cord.nvim?tab=readme-ov-file#-themes).

```lua
display = {
  theme = 'default',   -- 'default', 'atom', 'catppuccin', 'minecraft', 'void', 'classic'
  flavor = 'accent',   -- 'dark', 'light', 'accent'
}
```

### Choosing which icons show up

`display.view` controls the large/small images:

```lua
display = {
  view = 'full',      -- both the file/asset icon and the editor icon (default)
  -- view = 'asset',  -- only the file/asset icon
  -- view = 'editor', -- only the editor icon
  -- view = 'auto',   -- both, but drop the file icon in new/empty buffers
}
```

**`full`**:


![Full view](https://github.com/vyfor/icons/blob/master/.github/assets/demo_view_both.png?raw=true)

**`asset`**:


![Asset view](https://github.com/vyfor/icons/blob/master/.github/assets/demo_view_asset.png?raw=true)

**`editor`**:


![Editor view](https://github.com/vyfor/icons/blob/master/.github/assets/demo_view_editor.png?raw=true)

### Swapping the icon and field layout

```lua
display = {
  swap_icons = true,  -- editor icon becomes the large image, file icon the small one
  swap_fields = true, -- show the workspace name above the file name
}
```

| Default | Swapped |
|--------|-------|
![Before](https://github.com/vyfor/icons/blob/master/.github/assets/demo_regular.png?raw=true) | ![After](https://github.com/vyfor/icons/blob/master/.github/assets/demo_swap.png?raw=true)

### Customizing the idle icon

The idle icon accepts any of Cord's built-in icons (across any theme/flavor) or
a direct URL:

```lua
opts = function()
  local icon = require('cord.api.icon')

  return {
    idle = {
      -- a different built-in icon from your current theme
      icon = icon.get('keyboard'),
      -- ...or borrow the idle icon from another theme
      -- icon = icon.get('idle', 'atom'),
      -- ...or another theme AND flavor
      -- icon = icon.get('idle', 'atom', 'light'),
      -- ...or a direct URL
      -- icon = 'https://example.com/my-idle-icon.png',
    },
  }
end
```

## 📝 Customizing Text

The `text` table controls the two lines of text in your presence. Each entry
can be a **string**, a **function**, a **string template**, or a **boolean**.

### Simple static text

```lua
text = {
  editing = 'Editing a file',
  viewing = 'Reading some code',
  workspace = 'Working on a project',
}
```

### Dynamic text with functions

Functions receive [`opts`](./Configuration.md#options-table) and return a string.

```lua
text = {
  editing = function(opts) return 'Editing ' .. opts.filename end,
  workspace = function(opts) return 'Project: ' .. opts.workspace end,
  terminal = function(opts) return 'In a terminal (' .. opts.name .. ')' end,
}
```

### Showing the cursor position

```lua
text = {
  editing = function(opts)
    return string.format('Editing %s:%d:%d', opts.filename, opts.cursor_line, opts.cursor_char)
  end,
}
```

### Marking modified (unsaved) buffers

```lua
text = {
  editing = function(opts)
    local text = 'Editing ' .. opts.filename
    if vim.bo.modified then text = text .. ' [+]' end
    return text
  end,
}
```

### Hiding or collapsing lines

The return value of a text entry has special meanings:

| Value         | Effect                                                          |
|---------------|-----------------------------------------------------------------|
| `''`          | Omit that line (the presence shows a single line of text)       |
| `false`       | Hide the activity entirely for this buffer type                 |
| `true`        | Leave the current activity unchanged (ignore this buffer)       |

```lua
text = {
  workspace = '',          -- drop the workspace line, keep just the file line
  games = function() end,  -- returning nil behaves like ''

  plugin_manager = false,  -- hide presence while in a plugin manager
  file_browser = true,     -- ignore file browsers; keep showing the previous activity
}
```

### String templates with variables

`variables = true` should be set before using string templates.

```lua
{
  variables = true,
  text = {
    editing = 'Editing ${filename}',
    workspace = 'In ${workspace}',
  },
}
```

You can also define your own variables:

```lua
{
  variables = {
    random = function() return math.random(1, 100) end,
  },
  text = {
    workspace = 'Rolled a ${random}',
  },
}
```

### One default for everything

`text.default` fills in every category you didn't explicitly set:

```lua
text = {
  default = 'Using Neovim', -- applies to all unset categories
  workspace = function(opts) return 'In ' .. opts.workspace end, -- overrides the default
}
```

## 🕹️ Buttons

You can show up to **two** buttons. Each button needs a `label` and a `url`.

### A static button

```lua
buttons = {
  {
    label = 'My Website',
    url = 'https://example.com',
  },
}
```

### Dynamic label and URL

```lua
buttons = {
  {
    label = function(opts)
      return opts.repo_url and 'View Repository' or 'My Website'
    end,
    url = function(opts)
      return opts.repo_url or 'https://example.com'
    end,
  },
}
```

### Hiding a button conditionally

If a button's either field returns `nil`, that button is dropped. For example, hiding the repository button when the instance is idle:

```lua
buttons = {
  {
    label = 'View Repository',
    url = function(opts)
      if opts.is_idle then return end -- no button while idle
      return opts.repo_url
    end,
  },
}
```

## 💤 Idle Behavior

Cord switches to an idle presence after `idle.timeout` milliseconds of
inactivity. You can fully customize what it shows.

### Custom idle messages

```lua
idle = {
  details = function(opts)
    return 'Taking a break from ' .. opts.workspace
  end,
  state = 'Be right back',
  tooltip = '😴',
}
```

### Hiding the presence entirely while idle

```lua
idle = {
  show_status = false, -- clear the presence instead of showing an idle status
}
```

## 🗃️ Custom Assets

Defining new or overriding existing assets.

### Replace an icon by extension

```lua
assets = {
  ['.rs'] = 'https://example.com/my-rust-icon.png',
}
```

### Adding a missing filetype

```lua
assets = {
  myfiletype = {
    name = 'Plugin Name',
    icon = '...',
    tooltip = 'My custom filetype',
    type = 'file_browser',  -- categorize it
  },
}
```

See [Assets Wiki](./Assets.md) for more details and examples.

## 🙈 Hiding Presence for Certain Projects

To keep certain workspaces hidden from your presence, use the [Visibility extension](./Extensions.md#-visibility):

```lua
require('cord').setup {
  extensions = {
    visibility = {
      rules = {
        blacklist = {
          'secret',                                  -- matches a workspace name
          '~/work/private',                          -- matches a path
          { type = 'glob', value = '**/vendor/**' }, -- matches a glob
        },
      },
    },
  },
}
```

## 🪝 Going Further with Hooks

[Hooks](./Configuration.md#-hooks) let you react to certain events. `post_activity`, for example, hands you the finished [`activity`](./Configuration.md#activity-options) before it's sent.

### Show the Neovim version in the small icon's tooltip

```lua
hooks = {
  post_activity = function(opts, activity)
    local v = vim.version()
    activity.assets.small_text = string.format('Neovim %d.%d.%d', v.major, v.minor, v.patch)
  end,
}
```

### Make certain fields clickable

Text:

```lua
hooks = {
  post_activity = function(opts, activity)
    if opts.repo_url then
      activity.details_url = opts.repo_url
    end
  end,
}
```

Images:

```lua
hooks = {
  post_activity = function(opts, activity)
    if opts.repo_url then
      activity.assets.large_url = opts.repo_url
    end
  end,
}
```

### Change the activity type

By default Cord shows up as *"Playing Neovim"*. You can switch it to *"Listening to ..."*, *"Watching ..."*, or *"Competing in ..."* with:

```lua
hooks = {
  post_activity = function(opts, activity)
    activity.type = 'playing' -- 'playing' | 'listening' | 'watching' | 'competing'
  end,
}
```

### Change the shown activity field

Rich Presence can be configured to show what appears on your status line:
- Application Name: *"Playing Neovim"* (default)
- Details: *"Editing main.lua"*
- State: *"In workspace"*

![Member list](https://github.com/vyfor/icons/blob/master/.github/assets/demo_profile.png?raw=true)

```lua
hooks = {
  post_activity = function(opts, activity)
    activity.status_display_type = 'name' -- 'name' | 'details' | 'state'
  end,
}
```

## ⏳ Going Further with Async

For anything that touches the filesystem or runs a command, you should use Cord's [async runtime](./Async.md) to avoid blocking the UI thread.
Do not forget to cache the output, as Cord re-runs your config frequently.

### Show the current Git branch

```lua
local async = require('cord.core.async')
local process = require('cord.core.uv.process')
 
require('cord').setup {
  variables = {
    git_branch = async.wrap(function(opts)
      -- run git command only once every 30 seconds
      return opts.cache:get_or_compute(opts.workspace_dir .. ':branch', 30, function()
        local result, err = process.spawn({
          cmd = 'git',
          args = { 'branch', '--show-current' },
          cwd = opts.workspace_dir,
        }):await()
 
        if err or result.code ~= 0 then
          -- we must return a non-nil value
          return false
        end
        return vim.trim(result.stdout)
      end)
    end),
  },
  text = {
    -- use the variable if available, otherwise fallback
    workspace = async.wrap(function(opts)
      local branch = opts:git_branch():await()
      if branch then return string.format('In %s (%s)', opts.workspace, branch) end
      return string.format('In %s', opts.workspace)
    end)
  },
}
```

### Hide the repository button for private repositories

```lua
buttons = {
  {
    label = 'View Repository',
    url = async.wrap(function(opts)
      local is_private = opts.cache:get_or_compute(
        opts.workspace_dir .. ':is_repo_private',
        300,
        function()
          local result = processd 
            .spawn({
              cmd = 'gh',
              args = { 'repo','view', '--json', 'isPrivate', '--template', '{{.isPrivate}}' },
              cwd = opts.workspace_dir,
            })
            :await()

          if not result or result.code ~= 0 then return false end
          return vim.trim(result.stdout)
        end
      )

      if is_private == 'true' then return end

      return opts.repo_url
    end),
  },
}
```

### Commits ahead upstream

```lua
variables = {
  git_ahead = async.wrap(function(opts)
    return opts.cache:get_or_compute(opts.workspace_dir .. ':ahead', 30, function()
      local result, err = process.spawn({
        cmd = 'git',
        args = { 'rev-list', '--count', '@{upstream}..HEAD' },
        cwd = opts.workspace_dir,
      }):await()

      if err or result.code ~= 0 then return false end
      return tonumber(vim.trim(result.stdout)) or 0
    end)
  end),
},
text = {
  workspace = async.wrap(function(opts)
    local ahead = opts:git_ahead():await()
    if ahead and ahead > 0 then return string.format('In %s (↑ %d)', opts.workspace, ahead) end
    return 'In ' .. opts.workspace
  end),
},
```

> [!IMPORTANT]
> See the [Async Wiki](./Async.md) for an in-depth explanation, including caching and error handling.

Share your snippets in [GitHub Discussions](https://github.com/vyfor/cord.nvim/discussions) or our [Discord community](https://discord.gg/q9rC4bjCHv)!
