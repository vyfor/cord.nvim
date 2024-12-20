# 🎨 Customization Examples

This guide showcases various creative ways to customize your Discord presence using Cord's powerful configuration system.

### Cursor Position
```lua
text = {
  editing = function(opts)
    return string.format('Editing %s - %s:%s', opts.filename, opts.cursor_line, opts.cursor_char)
  end
}
```

### Workspace Blacklist
```lua
local blacklist = {
  'blacklisted_workspace',
  'another_blacklisted_workspace'
}

local is_blacklisted = function(opts)
  return vim.tbl_contains(blacklist, opts.workspace_name)
end

text = {
  viewing = function(opts)
    return is_blacklisted(opts) and 'Viewing a file' or ('Viewing ' .. opts.filename)
  end,
  editing = function(opts)
    return is_blacklisted(opts) and 'Editing a file' or ('Editing ' .. opts.filename)
  end,
}
```

### Git Branch & Status
```lua
local git_branch = vim.fn.system('git branch --show-current'):gsub('\n', '')

variables = {
  git_status = function(opts)
    return git_branch
  end
}

text = {
  editing = function(opts)
    return string.format('Editing %s - on branch %s', opts.filename, opts.git_status)
  end
}

hooks = {
  on_workspace_change = function(opts)
    git_branch = vim.fn.system('git branch --show-current'):gsub('\n', '')
  end
}
```

### Time-Based Status
```lua
text = {
  workspace = function(opts)
    local hour = tonumber(os.date('%H'))
    local status = 
      hour >= 22 and '🌙 Late night coding' or
      hour >= 18 and '🌆 Evening session' or
      hour >= 12 and '☀️ Afternoon coding' or
      hour >= 5 and '🌅 Morning productivity' or
      '🌙 Midnight hacking'
    
    return string.format('%s: %s', status, opts.filename)
  end
}
```

### LSP-Aware Status
```lua
local get_errors = function(bufnr) return vim.diagnostic.get(bufnr, { severity = { vim.diagnostic.severity.ERROR } }) end
local errors = get_errors(vim.api.nvim_get_current_buf()) -- pass the current buffer; pass nil to get errors for all buffers

vim.api.nvim_create_autocmd('DiagnosticChanged', {
  callback = function()
    errors = get_errors(vim.api.nvim_get_current_buf())
  end
})

text = {
  editing = function(opts)
    return string.format('Editing %s - %s errors', opts.filename, #errors)
  end
}
```

```lua
buttons = {
  {
    label = function(opts)
      return opts.repo_url and 'View Repository' or 'My Website'
    end
    url = function(opts)
      return opts.repo_url or 'https://example.com'
    end
  }
}
```

### Documentation Links
```lua
buttons = {
  {
    label = function(opts)
      local docs = {
        rust = 'Rust Docs',
        typescript = "TS Docs',
        lua = 'Lua Reference',
      }
      return docs[opts.filetype] or 'Documentation'
    end,
    url = function(opts)
      local urls = {
        rust = 'https://doc.rust-lang.org/std/',
        typescript = 'https://www.typescriptlang.org/docs/',
        lua = 'https://www.lua.org/manual/5.1/',
      }
      return urls[opts.filetype] or 'https://devdocs.io'
    end
  }
}
```

### Project-Based Idle Messages
```lua
idle = {
  details = function(opts)
    return string.format('Taking a break from %s', opts.workspace_name)
  end
}
```

### Random Quotes
```lua
local quotes = {
  'GTA VI came out before my Rust program finished compiling. ⏳',
  'When your code works on the first try. 😱',
  'It’s not a bug, it’s a feature. 🐛✨',
  'I don’t always test my code, but when I do, I do it in production. 💥',
  'My code works, I have no idea why. 🤷‍♂️',
  'Hello from the other side... of a merge conflict. 🔀',
  'If it works, don’t touch it. 🛑',
  'May your code never compile on the first try. 🤞',
}

hooks = {
  on_activity = function(opts, activity)
    activity.details = quotes[math.random(#quotes)]
  end
}
```
