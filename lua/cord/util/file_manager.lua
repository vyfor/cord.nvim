local utils = require 'cord.util'

local uv = vim.loop or vim.uv

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

function M.get_executable(pid, callback)
  local executable_name = M.get_executable_name()
  local target_path = M.get_target_path(executable_name)
  local data_path = M.get_data_path()
  local executable_path = data_path .. utils.path_sep .. executable_name

  uv.fs_stat(target_path, function(err)
    if not err then
      uv.fs_stat(executable_path, function(err)
        if not err then
          if pid then utils.kill_process(pid) end

          utils.rm_file(executable_path, function(err)
            if err then
              callback(
                nil,
                'Failed to remove existing executable: ' .. err,
                false
              )
              return
            end
            utils.move_file(target_path, executable_path, function(err)
              if err then
                callback(nil, 'Failed to move executable: ' .. err, false)
                return
              end
              callback(executable_path, nil, true)
            end)
          end)
        else
          utils.mkdir(data_path, function()
            utils.move_file(target_path, executable_path, function(err)
              if err then
                callback(nil, 'Failed to move executable: ' .. err, false)
                return
              end
              callback(executable_path, nil, true)
            end)
          end)
        end
      end)
    else
      uv.fs_stat(executable_path, function(err)
        if not err then
          callback(executable_path, nil, false)
        else
          callback(nil, 'Executable not found', false)
        end
      end)
    end
  end)
end

return M
