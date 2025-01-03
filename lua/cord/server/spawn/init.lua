local async = require 'cord.core.async'
local Future = require 'cord.core.async.future'
local M = {}

M.spawn = async.wrap(function(config, pipe_path)
  return Future.new(function(resolve, reject)
    local exec_path = config.advanced.server.executable_path
    local update_strategy = config.advanced.server.update
    local client_id = config.editor.client
    if not exec_path then
      exec_path = require('cord.server.fs').get_executable_path()
    end

    local fs = require 'cord.core.uv.fs'
    local stat = fs.stat(exec_path):get()
    if not stat then
      if update_strategy == 'fetch' then
        require('cord.server.update').fetch():await()
      elseif update_strategy == 'build' then
        require('cord.server.update').build():await()
      else
        require('cord.plugin.log').error 'Could not find the server executable'
      end
      return resolve(false, false)
    end

    local process = require 'cord.core.uv.process'
    process.spawn_daemon {
      cmd = exec_path,
      args = {
        '-p',
        pipe_path,
        '-c',
        client_id,
        '-t',
        config.advanced.server.timeout,
        '-r',
        config.advanced.discord.reconnect.enabled
            and config.advanced.discord.reconnect.interval
          or 0,
      },
      on_stdout = function(data)
        if data:match 'Ready' then resolve(true, false) end
      end,
      on_stderr = function(err)
        if err:match 'another instance is running' then
          resolve(true, true)
          return
        end

        reject(err)
      end,
      on_error = function(err) reject(err) end,
    }
  end)
end)

return M
