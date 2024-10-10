local cord = {}

local ffi = require 'ffi'
local utils = require 'cord.utils'
local logger = require 'cord.log'

cord.config = {
  usercmds = true,
  log_level = 'error',
  timer = {
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
    swap_fields = false,
    swap_icons = false,
    workspace_blacklist = {},
  },
  lsp = {
    show_problem_count = false,
    severity = 1,
    scope = 'workspace',
  },
  idle = {
    enable = true,
    show_status = true,
    timeout = 300000,
    disable_on_focus = false,
    text = 'Idle',
    tooltip = '💤',
  },
  text = {
    viewing = 'Viewing {}',
    editing = 'Editing {}',
    file_browser = 'Browsing files in {}',
    plugin_manager = 'Managing plugins in {}',
    lsp_manager = 'Configuring LSP in {}',
    vcs = 'Committing changes in {}',
    workspace = 'In {}',
  },
  buttons = {
    {
      label = 'View Repository',
      url = 'git',
    },
  },
  assets = nil,
}

local discord
local timer = vim.loop.new_timer()
local enabled = false
local is_focused = true
local force_idle = false
local problem_count = -1
local last_updated = os.clock()
local last_presence
local log_level
local callbacks

-- Must be wrapped in vim.schedule
local function init(config)
  local blacklist_len = #config.display.workspace_blacklist
  local blacklist_arr = ffi.new(
    'const char*[' .. blacklist_len .. ']',
    config.display.workspace_blacklist
  )

  local first_url = config.buttons[1] and config.buttons[1].url
  local second_url = config.buttons[2] and config.buttons[2].url

  if
    not first_url or first_url == 'git' and not config.display.show_repository
  then
    first_url = ''
  end

  if
    not second_url
    or second_url == 'git' and not config.display.show_repository
  then
    second_url = ''
  end

  return discord.init(
    ffi.new(
      'InitArgs',
      config.editor.client,
      config.editor.image,
      config.editor.tooltip,
      config.idle.text,
      config.idle.tooltip,
      config.text.viewing,
      config.text.editing,
      config.text.file_browser,
      config.text.plugin_manager,
      config.text.lsp_manager,
      config.text.vcs,
      config.text.workspace,
      blacklist_arr,
      blacklist_len,
      vim.fn.getcwd(),
      config.display.swap_fields,
      config.display.swap_icons,
      log_level
    ),
    ffi.new(
      'Buttons',
      (config.buttons[1] and config.buttons[1].label) or '',
      first_url,
      (config.buttons[2] and config.buttons[2].label) or '',
      second_url
    ),
    not enabled
        and callbacks
        and ffi.new('Callbacks', callbacks[1], callbacks[2])
      or nil
  )
end

local function should_update_presence(current_presence)
  local presence = last_presence
  return not presence
    or current_presence.cursor_line ~= presence.cursor_line
    or current_presence.cursor_col ~= presence.cursor_col
    or current_presence.name ~= presence.name
    or current_presence.type ~= presence.type
    or current_presence.readonly ~= presence.readonly
    or current_presence.problem_count ~= presence.problem_count
end

local function update_idle_presence(config)
  local presence = last_presence
  if presence and presence['idle'] then return false end

  if force_idle then
    last_presence = {
      idle = true,
    }
    if config.timer.reset_on_idle then discord.update_time() end
    if config.idle.show_status then
      discord.update_presence(
        ffi.new('PresenceArgs', '', 'Cord.idle', nil, 0, false)
      )
    else
      discord.clear_presence()
    end
  end

  if
    config.idle.enable
    and (
      config.idle.timeout == 0
      or (os.clock() - last_updated) * 1000 >= config.idle.timeout
    )
  then
    if config.idle.disable_on_focus and is_focused then return false end
    last_presence = {
      idle = true,
    }
    if config.display.show_time and config.timer.reset_on_idle then
      discord.update_time()
    end
    if config.idle.show_status then
      discord.update_presence(
        ffi.new('PresenceArgs', '', 'Cord.idle', nil, 0, false)
      )
    else
      discord.clear_presence()
    end
  end
end

local function update_presence(config)
  local cursor = vim.api.nvim_win_get_cursor(0)
  problem_count = utils.get_problem_count(config) or -1
  local current_presence = {
    name = vim.fn.expand '%:t',
    type = vim.bo.filetype,
    readonly = vim.bo.readonly,
    cursor_line = cursor[1],
    cursor_col = cursor[2] + 1,
    problem_count = problem_count,
  }

  if current_presence.type == '' then
    if current_presence.name == '' then
      current_presence.type = 'Cord.new'
    else
      current_presence.type = 'Cord.unknown'
    end
  end

  if should_update_presence(current_presence) then
    force_idle = false
    last_updated = os.clock()
    if config.display.show_time and config.timer.reset_on_change then
      discord.update_time()
    end
    local cursor_pos = config.display.show_cursor_position
        and (current_presence.cursor_line ~= 1 or current_presence.cursor_col ~= 1)
        and (current_presence.cursor_line .. ':' .. current_presence.cursor_col)
      or nil

    if current_presence.type == 'toggleterm' then
      local type = current_presence.name:match ':?%s-([^:]-)%s-&?::toggleterm'
        or current_presence.name:match ':?%s-([^:]-)%s-&?;#toggleterm'

      if type ~= nil and type ~= '' then
        current_presence.name = type
        current_presence.type = type
      end
    end

    local icon, name =
      utils.get_icon(config, current_presence.name, current_presence.type)

    local success
    if icon then
      success = discord.update_presence_with_assets(
        icon.name or '',
        name,
        type(icon) == 'string' and icon or icon.icon,
        icon.tooltip,
        icon.type or 0,
        ffi.new(
          'PresenceArgs',
          current_presence.name,
          current_presence.type,
          cursor_pos,
          problem_count,
          current_presence.readonly
        )
      )
    else
      success = discord.update_presence(
        ffi.new(
          'PresenceArgs',
          current_presence.name,
          current_presence.type,
          cursor_pos,
          problem_count,
          current_presence.readonly
        )
      )
    end

    if success then last_presence = current_presence end
  else
    update_idle_presence(config)
  end
end

local function connect(config)
  timer:stop()
  timer:start(
    0,
    config.timer.interval,
    vim.schedule_wrap(function() update_presence(config) end)
  )
end

local function start_timer(config)
  timer:stop()
  if not utils.validate_severity(config) then return end
  if config.display.show_time then discord.update_time() end

  connect(config)
  enabled = true
end

function cord.setup(userConfig)
  if vim.fn.has 'nvim-0.5' ~= 1 then
    logger.error 'Cord requires Neovim 0.5 or higher'
    return
  end

  if vim.g.cord_initialized == nil then
    local config = vim.tbl_deep_extend('force', cord.config, userConfig or {})
    config.timer.interval = math.max(config.timer.interval, 500)
    log_level = logger.init(config.log_level)

    discord = utils.init_discord(ffi)
    if not discord then return end

    cord.setup_autocmds(config)
    if config.usercmds then cord.setup_usercmds(config) end

    vim.cmd [[
      autocmd! ExitPre * lua require('cord').disconnect()
    ]]

    vim.g.cord_initialized = true

    vim.schedule(function()
      if not callbacks then
        callbacks = {
          ffi.cast('void (*)(const char*, int)', function(message, level)
            local res, msg = pcall(ffi.string, message)
            if res then pcall(vim.notify, msg, level) end
          end),
          ffi.cast('void (*)()', function() pcall(cord.cleanup) end),
        }
      end

      local status = init(config)
      if status < 0 then
        return
      elseif status > 0 then
        cord.disconnect()
        return
      end

      start_timer(config)
    end)
  end
end

function cord.setup_autocmds(config)
  vim.cmd [[
    autocmd! DirChanged * lua require('cord').on_dir_changed()
    autocmd! FocusGained * lua require('cord').on_focus_gained()
    autocmd! FocusLost * lua require('cord').on_focus_lost()
  ]]

  function cord.on_dir_changed()
    last_presence = nil
    if not discord.update_workspace(vim.fn.getcwd()) then
      timer:stop()
      discord.clear_presence()
      enabled = false
    else
      if not enabled then start_timer(config) end
    end
  end

  function cord.on_focus_gained()
    is_focused = true
    last_presence = nil
  end

  function cord.on_focus_lost() is_focused = false end
end

function cord.setup_usercmds(config)
  vim.cmd [[
    command! CordConnect lua require('cord').connect()
    command! CordReconnect lua require('cord').reconnect()
    command! CordDisconnect lua require('cord').disconnect()
    command! CordTogglePresence lua require('cord').toggle_presence()
    command! CordShowPresence lua require('cord').show_presence()
    command! CordHidePresence lua require('cord').hide_presence()
    command! CordToggleIdle lua require('cord').toggle_idle()
    command! CordIdle lua require('cord').idle()
    command! CordUnidle lua require('cord').unidle()
    command! -nargs=1 CordWorkspace lua require('cord').set_workspace(<f-args>)
  ]]

  function cord.connect()
    if discord.is_connected() then return end

    vim.defer_fn(function()
      local status = init(config)
      if status < 0 then
        return
      elseif status > 0 then
        cord.disconnect()
        return
      end

      if not enabled then start_timer(config) end
    end, 100)
  end

  function cord.reconnect()
    cord.disconnect()
    cord.connect()
  end

  function cord.toggle_presence()
    if enabled then
      timer:stop()
      discord.clear_presence()
      enabled = false
      last_presence = nil
    else
      start_timer(config)
    end
  end

  function cord.show_presence()
    if not enabled then start_timer(config) end
  end

  function cord.hide_presence()
    timer:stop()
    discord.clear_presence()
    enabled = false
    last_presence = nil
  end

  function cord.toggle_idle()
    if last_presence['idle'] then
      force_idle = false
      last_updated = os.clock()
      last_presence = nil
    else
      force_idle = true
    end
  end

  function cord.idle() force_idle = true end

  function cord.unidle()
    force_idle = false
    last_updated = os.clock()
    last_presence = nil
  end

  function cord.set_workspace(workspace)
    last_presence = nil
    if not discord.set_workspace(workspace) then
      timer:stop()
      discord.clear_presence()
      enabled = false
    else
      if not enabled then start_timer(config) end
    end
  end
end

function cord.disconnect()
  timer:stop()
  discord.disconnect()
  enabled = false
  last_presence = nil
end

function cord.cleanup()
  timer:stop()
  enabled = false
  last_presence = nil
end

return cord
