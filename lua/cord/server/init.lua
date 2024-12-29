local async = require 'cord.core.async'
local logger = require 'cord.plugin.log'

local M = {}

function M:connect(path, retried)
  return async.wrap(function()
    if M.is_updating then
      logger.debug 'Operation canceled: Server is updating'
      return
    end

    logger.info 'Connecting...'
    logger.debug('Pipe: ' .. path)
    M.client = require('cord.core.uv.pipe').new()
    local _, err = M.client:connect(path):get()

    if not err then
      logger.debug 'Connected to pipe'
      return M:run():await()
    end

    if retried then
      logger.error('Failed to connect to pipe: ' .. err)
      return
    end

    if err ~= 'ENOENT' and err ~= 'ECONNRESET' then
      if err == 'ECONNREFUSED' or err == 'ETIMEDOUT' then
        logger.debug 'Found stale pipe. Removing...'
        require('cord.core.uv.fs').unlink(path):get()
        goto spawn
      end

      logger.error('Failed to connect to pipe: ' .. err)
      return
    end

    ::spawn::
    logger.debug 'Pipe not found. Spawning server executable...'

    local process = require 'cord.server.spawn'
    process
      .spawn(
        self.config.editor.client,
        path,
        self.config.advanced.server.executable_path
      )
      :and_then(function(retry)
        async.run(function()
          logger.debug 'Server executable spawned'
          if retry then return M:connect(path):await() end
          M:connect(path, true):await()
        end)
      end, function(err) logger.error(err) end)
  end)()
end

function M:run()
  return async.wrap(function()
    local EventSender = require 'cord.server.event.sender'
    local EventReceiver = require 'cord.server.event.receiver'
    M.tx = EventSender.new(M.client)
    M.rx = EventReceiver.new(M.client)

    M.rx:register(
      'ready',
      vim.schedule_wrap(function()
        async.run(function()
          logger.info 'Connected to Discord'
          M.tx:initialize(self.config)

          local ActivityManager = require 'cord.plugin.activity.manager'
          local manager =
            ActivityManager.new({ tx = M.tx, config = self.config }):await()

          M.client.on_close = vim.schedule_wrap(function()
            if M.manager then M.manager:cleanup() end
            if self.config.hooks.on_disconnect then
              self.config.hooks.on_disconnect()
            end
          end)

          manager:run()
          M.manager = manager
        end)
      end)
    )

    M.rx:run()
    logger.debug 'Server initialized'
  end)()
end

function M:initialize(config)
  self.config = config or self.config
  async.run(function()
    logger.debug 'Initializing server...'

    local path = self.config.advanced.server.pipe_path
      or require('cord.plugin.constants').get_pipe_path()

    M:connect(path):await()
  end)
end

return M
