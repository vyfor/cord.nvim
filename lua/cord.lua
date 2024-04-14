local cord = {}

local ffi = require('ffi')
local utils = require('cord.utils')

cord.config = {
  usercmds = true,
  timer = {
    enable = true,
    interval = 1500,
    reset_on_idle = false,
    reset_on_change = false,
  },
  editor = {
    image = nil,
    client = 'neovim',
    tooltip = 'The Superior Text Editor',
  },
  display = {
    show_time = true,
    show_repository = true,
    show_cursor_position = false,
  },
  lsp = {
    show_problem_count = false,
    severity = 1,
    scope = 'workspace',
  },
  idle = {
    show_idle = true,
    timeout = 300000,
    disable_on_focus = true,
    text = 'Idle',
    tooltip = '💤',
  },
  text = {
    viewing = 'Viewing {}',
    editing = 'Editing {}',
    file_browser = 'Browsing files in {}',
    plugin_manager = 'Managing plugins in {}',
    workspace = 'In {}',
  },
  buttons = {
    {
      label = 'View Repository',
      url = 'git',
    }
  }
}

local discord
local timer = vim.loop.new_timer()
local enabled = false
local is_focused = true
local problem_count = -1
local last_updated = os.clock()
local last_presence

local function connect(config)
  discord.init(
    config.editor.client,
    config.editor.image,
    config.editor.tooltip,
    config.idle.text,
    config.idle.tooltip,
    config.text.viewing,
    config.text.editing,
    config.text.file_browser,
    config.text.plugin_manager,
    config.text.workspace
  )
end

local function should_update_presence(current_presence)
  return not last_presence or
    current_presence.cursor_line ~= last_presence.cursor_line or
    current_presence.cursor_col ~= last_presence.cursor_col or
    current_presence.name ~= last_presence.name or
    current_presence.type ~= last_presence.type or
    current_presence.readonly ~= last_presence.readonly or
    current_presence.problem_count ~= last_presence.problem_count
end

local function update_idle_presence(config)
  if last_presence['idle'] then
    return false
  end
  if config.idle.show_idle and (config.idle.timeout == 0 or (os.clock() - last_updated) * 1000 >= config.idle.timeout) then
    if config.idle.disable_on_focus and is_focused then
      return false
    end
    last_presence['idle'] = true
    if config.display.show_time and config.timer.reset_on_idle then
      discord.set_time()
    end
    discord.update_presence('', 'Cord.idle', false, nil, 0)
    return true
  end
  return false
end

local function update_presence(config)
  local cursor = vim.api.nvim_win_get_cursor(0)
  problem_count = utils.get_problem_count(config) or -1
  local current_presence = {
    name = vim.fn.expand('%:t'),
    type = vim.bo.filetype,
    readonly = vim.bo.readonly,
    cursor_line = cursor[1],
    cursor_col = cursor[2] + 1,
    problem_count = problem_count
  }

  if should_update_presence(current_presence) then
    last_updated = os.clock()
    if config.display.show_time and config.timer.reset_on_change then
      discord.set_time()
    end
    local cursor_pos = config.display.show_cursor_position and (current_presence.cursor_line .. ':' .. current_presence.cursor_col) or nil
    local success = discord.update_presence(current_presence.name, current_presence.type, current_presence.readonly, cursor_pos, problem_count)
    if success then
      last_presence = current_presence
    end
  elseif not update_idle_presence(config) then
    return
  end
end

local function start_timer(config)
  timer:stop()
  if vim.g.cord_started == nil then
    vim.g.cord_started = true
    if not utils.validate_severity(config) then return end
    utils.update_cwd(config, discord)
    cord.setup_autocmds(config)
    if config.display.show_time then
      discord.update_time()
    end
  end
  timer:start(0, config.timer.interval, vim.schedule_wrap(function() update_presence(config) end))
end

function cord.setup(userConfig)
  if vim.g.cord_initialized == nil then
    local config = vim.tbl_deep_extend('force', cord.config, userConfig or {})
    config.timer.interval = math.max(config.timer.interval, 500)

    local work = vim.loop.new_async(vim.schedule_wrap(function()
      discord = utils.init_discord(ffi)
      connect(config)
      if config.timer.enable then
        start_timer(config)
      end

      vim.api.nvim_create_autocmd('ExitPre', { callback = function() discord.disconnect() end })
      if config.usercmds then cord.setup_usercmds(config) end
    end))
    work:send()
    vim.g.cord_initialized = true
  end
end

function cord.setup_autocmds(config)
  vim.api.nvim_create_autocmd('DirChanged', { callback = function() utils.update_cwd(config, discord) end })
  vim.api.nvim_create_autocmd('FocusGained', { callback = function() is_focused = true; last_presence = nil end })
  vim.api.nvim_create_autocmd('FocusLost', { callback = function() is_focused = false end })
end

function cord.setup_usercmds(config)
  vim.api.nvim_create_user_command('CordConnect', function()
    connect(config)
    start_timer(config)
  end, {})

  vim.api.nvim_create_user_command('CordReconnect', function()
    timer:stop()
    discord.disconnect()
    last_presence = nil
    connect(config)
    start_timer(config)
    enabled = true
  end, {})

  vim.api.nvim_create_user_command('CordDisconnect', function()
    timer:stop()
    discord.disconnect()
    enabled = false
    last_presence = nil
  end, {})

  vim.api.nvim_create_user_command('CordTogglePresence', function()
    if enabled then
      timer:stop()
      discord.clear_presence()
      enabled = false
      last_presence = nil
    else
      start_timer(config)
      enabled = true
    end
  end, {})

  vim.api.nvim_create_user_command('CordShowPresence', function()
    start_timer(config)
    enabled = true
  end, {})

  vim.api.nvim_create_user_command('CordHidePresence', function()
    timer:stop()
    discord.clear_presence()
    enabled = false
    last_presence = nil
  end, {})
end

return cord
