local async = require 'cord.core.async'
local Future = require 'cord.core.async.future'
local M = {}

M.spawn = async.wrap(function(config, pipe_path)
  return Future.new(function(resolve, reject)
    local logger = require 'cord.plugin.log'
    local update_strategy = config.advanced.server.update
    local client_id = config.editor.client
    local exec_path = require('cord.server.fs').get_executable_path(config)

    local fs = require 'cord.core.uv.fs'
    local stat = fs.stat(exec_path):get()
    if not stat then
      logger.debug('Spawn: executable missing at ' .. tostring(exec_path))
      if update_strategy == 'fetch' then
        require('cord.server.update').fetch():await()
      elseif update_strategy == 'install' then
        require('cord.server.update').install():await()
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
        config.advanced.discord.reconnect.enabled and config.advanced.discord.reconnect.interval
          or 0,
        config.advanced.discord.reconnect.initial and '-i' or nil,
      },
      on_stdout = function(data)
        if data:match 'Ready' then
          logger.debug 'Spawn: server signaled Ready'
          resolve(true, false)
        end
      end,
      on_stderr = function(err)
        if err:match 'another instance is running' then
          logger.debug 'Spawn: another instance detected; will retry connect'
          resolve(true, true)
          return
        end

        logger.debug('Spawn: stderr: ' .. tostring(err))
        reject(err)
      end,
      on_error = function(err)
        logger.debug('Spawn: on_error: ' .. tostring(err))
        reject(err)
      end,
    }
  end)
end)

return M
