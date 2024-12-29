local async = require 'cord.core.async'
local Future = require 'cord.core.async.future'
local M = {}

M.spawn = async.wrap(function(client_id, pipe_path, exec_path)
  if not exec_path then
    exec_path = require('cord.server.fs').get_executable_path()
  end

  local fs = require 'cord.core.uv.fs'
  if not fs.stat(exec_path):await() then
    error('Could not find server executable. Please update it', 0)
    return
  end

  return Future.new(function(resolve, reject)
    local process = require 'cord.core.uv.process'
    process.spawn_daemon {
      cmd = exec_path,
      args = {
        '-p',
        pipe_path,
        '-c',
        client_id,
        '-t',
        require('cord.plugin.config').opts.advanced.server.timeout,
      },
      on_stdout = function(data)
        if data:match 'Ready' then resolve(false) end
      end,
      on_stderr = function(err)
        if err:match 'another instance is running' then
          resolve(true)
          return
        end

        reject(err)
      end,
      on_error = function(err) reject(err) end,
    }
  end)
end)

return M
