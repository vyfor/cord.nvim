local M = {}

M.build = function()
  require('cord.core.async').run(function() require('cord.server.update').build():await() end)
end
M.fetch = function()
  require('cord.core.async').run(function() require('cord.server.update').fetch():await() end)
end
M.update = function()
  local mode = require('cord.plugin.config').advanced.server.update

  if mode == 'fetch' then
    M.fetch()
  elseif mode == 'build' then
    M.build()
  elseif mode ~= 'none' then
    require('cord.plugin.log').log_raw(
      vim.log.levels.ERROR,
      'Unknown update mode: ' .. '\'' .. mode .. '\''
    )
  end
end
M.show_presence = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:resume() end
end
M.hide_presence = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:hide() end
end
M.toggle_presence = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:toggle() end
end
M.idle = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:idle() end
end
M.force_idle = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:force_idle() end
end
M.unidle = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:unidle() end
end
M.toggle_idle = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:toggle_idle() end
end
M.clear_presence = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:clear_activity(true) end
end
M.restart = function()
  vim.schedule(function()
    local cord = require 'cord.server'
    if cord.is_updating then
      require('cord.plugin.log').info 'Operation canceled: Server is updating'
      return
    end

    require('cord.plugin.log').debug 'Restarting...'
    local function initialize()
      require('cord.core.async').run(function() cord:initialize() end)
    end

    if cord.manager then cord.manager:cleanup() end
    if not cord.tx then return initialize() end
    if not cord.client then return initialize() end
    if cord.client:is_closing() then return initialize() end
    if cord.client.on_close then cord.client.on_close() end

    cord.client.on_close = function()
      cord.client.on_close = nil
      initialize()
    end

    cord.tx:shutdown()
  end)
end
M.shutdown = function()
  local cord = require 'cord.server'

  if not cord.client or cord.client:is_closing() then
    return require('cord.plugin.log').info 'Server is not running'
  end

  cord.is_updating = false
  if cord.manager then cord.manager:cleanup() end
  cord.tx:shutdown()
  require('cord.plugin.log').info 'Stopped server'
end
M.status = function()
  local cord = require 'cord.server'
  if cord.status == 'ready' then
    require('cord.plugin.log').info 'Status: Connected to Discord'
  elseif cord.status == 'connecting' then
    require('cord.plugin.log').info 'Status: Connecting to server'
  elseif cord.status == 'connected' then
    require('cord.plugin.log').info 'Status: Connecting to Discord'
  else
    require('cord.plugin.log').info 'Status: Disconnected'
  end
end
M.check = function()
  require('cord.core.async').run(
    function() require('cord.server.update').check_version():await() end
  )
end
M.version = function()
  require('cord.core.async').run(function() require('cord.server.update').version():await() end)
end

M.features = {
  idle = { path = { 'idle', 'enabled' }, on_disable = function() M.unidle() end },
}

local function handle_feature(feature, enable)
  local feat = M.features[feature]
  if not feat then
    require('cord.plugin.log').log_raw(
      vim.log.levels.ERROR,
      'Unknown feature: \'' .. feature .. '\''
    )
    return
  end

  local config = require 'cord.plugin.config'
  local target = config
  for i = 1, #feat.path - 1 do
    target = target[feat.path[i]]
  end

  local last = feat.path[#feat.path]
  if enable == true then
    target[last] = true
    if feat.on_disable then feat.on_disable() end
  elseif enable == false then
    target[last] = false
    if feat.on_disable then feat.on_disable() end
  else
    target[last] = not target[last]
    if target[last] then
      if feat.on_enable then feat.on_enable() end
    else
      if feat.on_disable then feat.on_disable() end
    end
  end
end

M.commands = {
  presence = {
    default = M.toggle_presence,
    subcommands = {
      show = M.show_presence,
      hide = M.hide_presence,
      toggle = M.toggle_presence,
      clear = M.clear_presence,
    },
  },
  idle = {
    default = M.toggle_idle,
    subcommands = {
      show = M.idle,
      hide = M.unidle,
      toggle = M.toggle_idle,
      force = M.force_idle,
    },
  },
  update = {
    default = M.update,
    subcommands = {
      check = M.check,
      fetch = M.fetch,
      build = M.build,
    },
  },
  status = M.status,
  version = M.version,
  restart = M.restart,
  shutdown = M.shutdown,
  enable = function(feature) handle_feature(feature, true) end,
  disable = function(feature) handle_feature(feature, false) end,
  toggle = function(feature) handle_feature(feature) end,
}

M.get_commands = function()
  local cmds = {}
  for cmd, _ in pairs(M.commands) do
    table.insert(cmds, cmd)
  end
  return cmds
end

M.get_subcommands = function(cmd)
  local command = M.commands[cmd]
  if not command or not command.subcommands then return {} end

  local subcmds = {}
  for subcmd, _ in pairs(command.subcommands) do
    table.insert(subcmds, subcmd)
  end
  return subcmds
end

M.get_features = function()
  local feats = {}
  for feat, _ in pairs(M.features) do
    table.insert(feats, feat)
  end
  return feats
end

M.handle = function(q_args)
  local args = vim.split(q_args:gsub('"', ''), '%s+')
  local args_len = #args
  if args_len == 0 then
    require('cord.plugin.log').log_raw(vim.log.levels.ERROR, 'No command provided')
    return
  end

  local cmd = args[1]
  local command = M.commands[cmd]

  if not command then
    require('cord.plugin.log').log_raw(vim.log.levels.ERROR, 'Unknown command: \'' .. cmd .. '\'')
    return
  end

  local execute
  if type(command) == 'function' then
    execute = command
  else
    execute = command.default
  end

  if args_len == 1 then
    execute()
    return
  end

  local subcmd = args[2]

  if cmd == 'enable' or cmd == 'disable' or cmd == 'toggle' then
    execute(subcmd)
    return
  end

  if type(command) == 'table' and command.subcommands and command.subcommands[subcmd] then
    command.subcommands[subcmd]()
  else
    require('cord.plugin.log').log_raw(
      vim.log.levels.ERROR,
      'Unknown subcommand: \'' .. subcmd .. '\' for command \'' .. cmd .. '\''
    )
  end
end

return M
