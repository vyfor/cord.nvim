local utils = require 'cord.util'

local M = {}

function M.get_plugin_root()
  local source = debug.getinfo(1, 'S').source:sub(2)
  return vim.fn.fnamemodify(source, ':h:h:h:h')
end

function M.get_target_path(name)
  return M.get_plugin_root()
    .. utils.path_sep
    .. 'target'
    .. utils.path_sep
    .. 'release'
    .. utils.path_sep
    .. name
end

function M.get_data_path()
  return vim.fn.stdpath 'data' .. utils.path_sep .. 'cord'
end

function M.get_executable_name()
  return utils.os_name == 'Windows' and 'cord.exe' or 'cord'
end

function M.get_executable()
  local executable_name = M.get_executable_name()
  local target_path = M.get_target_path(executable_name)
  local data_path = M.get_data_path()
  local executable_path = data_path .. utils.path_sep .. executable_name

  if utils.file_exists(target_path) then
    if not utils.file_exists(executable_path) then
      utils.mkdir(data_path)
    else
      local ok, err = utils.rm_file(executable_path)
      if not ok then
        return nil, 'Failed to remove existing executable: ' .. (err or '')
      end
    end

    local ok, err = utils.move_file(target_path, executable_path)
    if not ok then return nil, 'Failed to move executable: ' .. (err or '') end
    return executable_path
  end

  if utils.file_exists(executable_path) then return executable_path end

  return nil, 'Executable not found'
end

return M
