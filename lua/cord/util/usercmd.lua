local commands = {
  build = function() require('cord.update').build() end,
  fetch = function() require('cord.update').fetch() end,
  show_presence = function() require('cord').manager:resume() end,
  hide_presence = function() require('cord').manager:hide() end,
  toggle_presence = function() require('cord').manager:toggle() end,
  idle = function() require('cord').manager:force_idle() end,
  unidle = function() require('cord').manager:unforce_idle() end,
  toggle_idle = function() require('cord').manager:toggle_idle() end,
  clear_presence = function() require('cord').manager:clear_activity() end,
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
