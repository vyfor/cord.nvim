local logger = require 'cord.util.logger'
local utils = require 'cord.util'

local uv = vim.loop or vim.uv

local M = {}

function M.spawn_server(config, path, callback)
  local executable
  if config.advanced.server.executable_path then
    executable = config.advanced.server.executable_path
  else
    executable = utils.os_name == 'Windows' and 'target/release/cord.exe'
      or 'target/release/cord'
  end

  if not utils.file_exists(executable) then
    logger.error('Server executable not found at \'' .. executable .. '\'')
    return
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
