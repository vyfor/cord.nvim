local commands = {
  build = function() require('cord.update').build() end,
  fetch = function() require('cord.update').fetch() end,
  show_presence = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:resume() end
  end,
  hide_presence = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:hide() end
  end,
  toggle_presence = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:toggle() end
  end,
  idle = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:force_idle() end
  end,
  unidle = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:unforce_idle() end
  end,
  toggle_idle = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:toggle_idle() end
  end,
  clear_presence = function()
    local cord = require 'cord'
    if cord.manager then cord.manager:clear_activity(true) end
  end,
  restart = function()
    local cord = require 'cord'

    if cord.producer then
      cord.producer:shutdown()
    elseif vim.g.cord_pid then
      require('cord.util').kill_process(vim.g.cord_pid)
    end

    cord.cleanup()
    cord.initialize()
  end,
}

local function handle(args)
  local command = commands[args[1]]

  if command then
    command()
  else
    error('Unknown command: ' .. '\'' .. args[1] .. '\'')
  end
end

return {
  handle = handle,
}
