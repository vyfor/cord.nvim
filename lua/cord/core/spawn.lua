local logger = require 'cord.util.logger'
local file_manager = require 'cord.util.file_manager'

local uv = vim.loop or vim.uv

local M = {}

local function spawn(executable, client, callback)
  local stdout = uv.new_pipe()
  local stderr = uv.new_pipe()
  uv.spawn(executable, {
    args = {
      '-p',
      client.path,
      '-c',
      client.config.editor.client,
      '-t',
      tostring(client.config.advanced.server.timeout),
    },
    stdio = { nil, stdout, stderr },
    detached = true,
    hide = true,
  })

  local stderr_data = ''
  stderr:read_start(function(err, chunk)
    if err then
      logger.error('Failed to read stderr: ' .. err)
      stderr:close()
      stdout:close()
      return
    end
    if chunk then
      stderr_data = stderr_data .. chunk
      if chunk:match 'kind: AlreadyExists' then
        callback()
        stderr:close()
        stdout:close()
        return
      end
      if stderr_data:match '\n$' then
        logger.error('Server error: ' .. stderr_data)
        stderr:close()
        stdout:close()
        return
      end
    end
  end)

  local stdout_data = ''
  stdout:read_start(function(err, chunk)
    if err then
      logger.error('Failed to read pipe: ' .. err)
      stdout:close()
      stderr:close()
      return
    end

    if chunk then
      stdout_data = stdout_data .. chunk
      if stdout_data:match 'Ready\n$' then
        callback()
        stdout:close()
        stderr:close()
        return
      end
    end
  end)

  return stdout, stderr
end

function M.spawn_server(client, callback)
  local executable = client.config.advanced.server.executable_path

  if not executable then
    file_manager.get_executable(
      client.config,
      nil,
      vim.schedule_wrap(function(executable_path, err)
        if err then
          logger.error(err)
          return
        end

        executable = executable_path
        spawn(executable, client, callback)
      end)
    )
  else
    spawn(executable, client, callback)
  end
end

return M
