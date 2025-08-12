# 🎨 Examples

Learn how to customize your Discord presence with examples. The possibilities are endless, and the only limit is your creativity!

### Customizing Icons

>[!IMPORTANT]
> If you use a plugin manager, avoid using `require` directly in tables; instead, use them within function initializers.

```lua
opts = function()
  return {
    display = {
      theme = 'default',
      flavor = 'dark',
    },
    idle = {
      -- change default idle icon to keyboard
      icon = require('cord.api.icon').get('keyboard'),
      -- or use another theme's idle icon
      icon = require('cord.api.icon').get('idle', 'atom'),
      -- or use another theme's idle icon from a different flavor
      icon = require('cord.api.icon').get('idle', 'atom', 'light'),
    }
  }
end
```

### Cursor Position
```lua
text = {
  editing = function(opts)
    return string.format('Editing %s - %s:%s', opts.filename, opts.cursor_line, opts.cursor_char)
  end
}
```

### Omitting Fields
```lua
text = {
  workspace = '',
  -- or
  workspace = function() end,
  -- or
  workspace = function() return '' end
}
```

### Neovim Version in Small Tooltip
```lua
hooks = {
  post_activity = function(opts, activity)
    local version = vim.version()
    activity.assets.small_text = string.format('Neovim %s.%s.%s', version.major, version.minor, version.patch)
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
  return vim.tbl_contains(blacklist, opts.workspace)
end

-- EITHER
-- A. use a custom text for the activity
text = {
  viewing = function(opts)
    return is_blacklisted(opts) and 'Viewing a file' or ('Viewing ' .. opts.filename)
  end,
  editing = function(opts)
    return is_blacklisted(opts) and 'Editing a file' or ('Editing ' .. opts.filename)
  end,
  workspace = function(opts)
    return is_blacklisted(opts) and 'In a secret workspace' or ('Working on ' .. opts.workspace)
  end
}

-- OR
-- B. simply hide the activity when in a blacklisted workspace
hooks = {
  workspace_change = function(opts)
    if is_blacklisted(opts) then
      opts.manager:hide()
    else 
      opts.manager:resume()
    end
  end
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
  workspace_change = function(opts)
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

### Dynamic Buttons
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

### Custom Idle Messages
```lua
idle = {
  details = function(opts)
    return string.format('Taking a break from %s', opts.workspace)
  end
}
```

### Hide Buttons on Idle
```lua
buttons = {
  {
    label = 'View Repository',
    url = function(opts)
      if not opts.is_idle then return opts.repo_url end
    end
  }
}
```

### Indicate Modified Buffers
```lua
text = {
  editing = function(opts)
    local text = 'Editing ' .. opts.filename
    if vim.bo.modified then text = text .. '[+]' end
    return text
  end,
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
  post_activity = function(_, activity)
    activity.details = quotes[math.random(#quotes)]
  end
}
```

### Last.fm Integration
```lua
local user, api_key, song = 'YOUR_USERNAME', 'YOUR_API_KEY', nil
local timer = vim.uv.new_timer()
timer:start(
  0,
  15000, -- look for new song every 15 seconds (default ratelimit on most Discord clients)
  vim.schedule_wrap(function()
    vim.system(
      { 'curl', 'http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user=' .. user .. '&api_key=' .. api_key .. '&format=json&limit=1, '-s', '--fail' },
      { text = true },
      vim.schedule_wrap(function(opts)
        if opts.code ~= 0 or not opts.stdout then
          print 'Failed to fetch data from last.fm'
          timer:close()
          return
        end

        local data = vim.fn.json_decode(opts.stdout)
        local recenttracks = data.recenttracks
        if not recenttracks then
          print 'Failed to fetch recenttracks from last.fm'
          timer:close()
          return
        end

        local track = recenttracks.track[1]
        if not track or not track['@attr'] then
          song = nil
          return
        end

        song = track.name
        if song == '' then song = nil end
      end)
    )
  end)
)

return {
  text = {
    workspace = function(opts)
      if song then return ('🎶 Playing ' .. song) end
      return opts.workspace
    end,
  },
}
```

### Keymappings
```lua
vim.keymap.set('n', '<leader>Ct', function() require('cord.api.command').toggle_presence() end)
vim.keymap.set('n', '<leader>Ci', function() require('cord.api.command').toggle_idle_force() end)
```