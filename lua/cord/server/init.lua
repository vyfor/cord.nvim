local async = require 'cord.core.async'
local logger = require 'cord.plugin.log'
local pipe = require 'cord.core.uv.pipe'
local config = require 'cord.plugin.config'

local M = {}

M.connect = async.wrap(function(path, retried)
  if M.is_updating then
    logger.debug 'Operation canceled: Server is updating'
    return
  end

  logger.info 'Connecting...'
  logger.debug('Pipe: ' .. path)
  M.client = pipe.new()
  local _, err = M.client:connect(path):get()

  if not err then
    logger.debug 'Connected to pipe'
    return M.run():await()
  end

  if retried then
    logger.error('Failed to connect to pipe: ' .. err)
    return
  end

  if err ~= 'ENOENT' then
    logger.error('Failed to connect to pipe: ' .. err)
    return
  end

  logger.debug 'Pipe not found. Spawning server executable...'

  local spawn = require 'cord.server.spawn'
  spawn
    .spawn(config.editor.client, path, config.advanced.server.executable_path)
    :and_then(function(retry)
      async.run(function()
        logger.debug 'Server executable spawned'
        if retry then return M.connect(path):await() end
        M.connect(path, true):await()
      end)
    end, function(err) logger.error(err) end)
end)

M.run = async.wrap(function()
  local EventSender = require 'cord.server.event.sender'
  local EventReceiver = require 'cord.server.event.receiver'
  M.tx = EventSender.new(M.client)
  M.rx = EventReceiver.new(M.client)

  M.rx:register(
    'ready',
    vim.schedule_wrap(function()
      async.run(function()
        logger.info 'Connected to Discord'
        M.tx:initialize(config)

        local ActivityManager = require 'cord.plugin.activity.manager'
        local manager = ActivityManager.new({ tx = M.tx, config = config })
          :await()

        M.client.on_close = vim.schedule_wrap(function()
          if M.manager then M.manager:cleanup() end
          if config.hooks.on_disconnect then config.hooks.on_disconnect() end
        end)

        manager:run()
        M.manager = manager
      end)
    end)
  )

  M.rx:run()
  logger.debug 'Server initialized'
end)

M.initialize = async.wrap(function()
  logger.debug 'Initializing server...'

  local path = config.advanced.server.pipe_path
  if not path then path = require('cord.plugin.constants').get_pipe_path() end

  return M.connect(path):await()
end)

return M
