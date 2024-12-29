local M = {}

M.build = function()
  require('cord.core.async').run(
    function() require('cord.server.update').build():await() end
  )
end
M.fetch = function()
  require('cord.core.async').run(
    function() require('cord.server.update').fetch():await() end
  )
end
M.update = function()
  local mode = require('cord.plugin.config').opts.advanced.server.update

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
  if cord.manager then cord.manager:force_idle() end
end
M.unidle = function()
  local cord = require 'cord.server'
  if cord.manager then cord.manager:unforce_idle() end
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

M.handle = function(args)
  local command = M[args[1]]

  if command then
    command()
  else
    require('cord.plugin.log').log_raw(
      vim.log.levels.ERROR,
      'Unknown command: ' .. '\'' .. args[1] .. '\''
    )
  end
end

return M
