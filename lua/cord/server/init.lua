local async = require 'cord.core.async'
local config = require 'cord.api.config'
local logger = require 'cord.api.log'

local M = {}

function M:connect(path, retried)
  return async.wrap(function()
    if M.is_updating then
      logger.debug 'Operation canceled: Server is updating'
      return
    end

    self.status = 'initializing'
    self.is_shut_down = false
    logger.debug 'Connecting to server...'

    logger.trace('Pipe: ' .. path)
    M.client = require('cord.core.uv.pipe').new()
    local _, err = M.client:connect(path):get()

    if not err then
      self.status = 'initialized'
      logger.debug 'Connected to server'

      return M:run():await()
    end

    if retried then error('Failed to connect to pipe: ' .. err, 0) end

    if err ~= 'ENOENT' and err ~= 'ECONNRESET' then
      if err == 'ECONNREFUSED' or err == 'ETIMEDOUT' then
        logger.debug 'Found stale pipe. Removing...'
        require('cord.core.uv.fs').unlink(path):get()
        goto spawn
      end

      error('Failed to connect to pipe: ' .. err, 0)
    end

    ::spawn::
    logger.debug 'Pipe not found. Spawning server executable...'

    local process = require('cord.server.spawn').spawn(config.get(), path)
    local should_continue, retry = process:await()
    if not should_continue then return end

    logger.debug 'Server executable spawned'
    if retry then return M:connect(path):await() end
    M:connect(path, true):await()
  end)()
end

function M:run()
  return async.wrap(function()
    M.tx = require('cord.server.ipc.sender').new(M.client)
    M.rx = require('cord.server.ipc.receiver').new(M.client)
    logger.debug 'Server: sending initialize event'
    M.tx:initialize(config.get())
    logger.debug 'Server: registering ready handler'
    M.rx:register(
      'status_update',
      false,
      vim.schedule_wrap(function(data)
        if data.status == 'connecting' then
          self.status = 'connecting'
          logger.debug 'Connecting to Discord...'
        elseif data.status == 'connected' then
          self.status = 'connected'
          logger.debug 'Handshaking with Discord...'
        elseif data.status == 'ready' then
          self.status = 'ready'
          async.run(function()
            logger.info 'Connected to Discord'

            local ActivityManager = require 'cord.internal.manager'
            local manager, err = ActivityManager.new({ tx = M.tx }):get()
            if not manager or err then
              self.status = 'disconnected'
              self.client:close()
              logger.error(err or 'Failed to initialize activity manager')
              return
            end

            M.client.on_close = vim.schedule_wrap(function()
              M.status = 'disconnected'
              M.manager:cleanup()

              if not self.is_shut_down then
                self.is_shut_down = true
                require('cord.internal.hooks').run 'shutdown'
              end
            end)

            manager:run()
            M.manager = manager
          end)
        elseif data.status == 'disconnected' then
          self.status = 'initialized'
          M.manager:cleanup()

          if not self.is_shut_down then
            self.is_shut_down = true
            require('cord.internal.hooks').run 'shutdown'
          end

          if config.advanced.discord.reconnect.enabled then logger.info 'Reconnecting...' end
        end
      end)
    )

    M.rx:register(
      'restart',
      false,
      vim.schedule_wrap(function()
        M:initialize()
      end)
    )

    logger.debug 'Server initialized; starting receiver'
    M.rx:run()
  end)()
end

function M:initialize()
  self.status = 'initializing'
  async.run(function()
    logger.debug 'Initializing server...'

    local path = config.advanced.server.pipe_path or require('cord.core.util').get_pipe_path()

    logger.trace(function() return 'Server pipe path: ' .. tostring(path) end)
    local _, err = M:connect(path):get()
    if err then
      self.status = 'disconnected'
      logger.error(err)
    end
  end)
end

function M:cleanup()
  if self.client then self.client:close() end
end

return M
