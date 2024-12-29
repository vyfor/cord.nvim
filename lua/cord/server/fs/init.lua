local async = require 'cord.core.async'

local M = {}

function M.get_plugin_root()
  local source = debug.getinfo(1, 'S').source:sub(2)
  return vim.fn.fnamemodify(source, ':h:h:h:h')
end

function M.get_data_path() return vim.fn.stdpath 'data' .. '/cord' end

function M.get_executable_path()
  return M.get_data_path() .. '/bin/' .. M.get_executable_name()
end

function M.get_executable_name()
  return require('cord.plugin.constants').get_os().name == 'windows'
      and 'cord.exe'
    or 'cord'
end

M.get_executable = async.wrap(function(config)
  local fs = require 'cord.core.uv.fs'
  local executable_path = M.get_executable_path()
  local stat = fs.stat(executable_path):await()

  if stat then
    return { path = executable_path, error = nil, needs_update = false }
  else
    local mode = config.advanced.server.update
    if mode == 'fetch' then
      local result = require('cord.server.update').fetch():await()
      return result
    elseif mode == 'build' then
      local result = require('cord.server.update').build():await()
      return result
    else
      error 'Executable not found'
    end
  end
end)

return M
