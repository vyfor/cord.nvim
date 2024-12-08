local logger = require 'cord.util.logger'
local utils = require 'cord.util'
local file_manager = require 'cord.util.file_manager'

local uv = vim.loop or vim.uv

local M = {}

function M.spawn_server(config, path, callback)
  local executable = config.advanced.server.executable_path
  if not executable then
    local executable_path, err = file_manager.get_executable()

    if err then
      logger.error(err)
      return
    end

    executable = executable_path
  end

  local stdout = uv.new_pipe()
  local stderr = uv.new_pipe()
  uv.spawn(executable, {
    args = {
      '-p',
      path,
      '-c',
      config.editor.client,
      '-t',
      tostring(config.advanced.server.timeout),
    },
    stdio = { nil, stdout, stderr },
    detached = true,
    hide = true,
  })

  stderr:read_start(vim.schedule_wrap(function(err, chunk)
    if err then
      logger.error('Failed to read stderr: ' .. err)
      return
    end
    if chunk then
      if chunk:match 'kind: AlreadyExists' then
        callback()
        stderr:close()
        stdout:close()
        return
      end
      logger.error('Server error: ' .. chunk)
    end
  end))

  stdout:read_start(vim.schedule_wrap(function(err, chunk)
    if err then
      logger.error('Failed to read pipe: ' .. err)
      return
    end

    if chunk and chunk:match 'Ready' then
      callback()
      return
    end
  end))
end

return M
