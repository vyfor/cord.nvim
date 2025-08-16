local async = require 'cord.core.async'
local config = require 'cord.plugin.config'
local logger = require 'cord.plugin.log'

local M = {}

function M:connect(path, retried)
  return async.wrap(function()
    if M.is_updating then
      logger.debug 'Operation canceled: Server is updating'
      return
    end

    self.status = 'connecting'
    logger.debug 'Connecting to pipe...'

    logger.trace('Pipe: ' .. path)
    M.client = require('cord.core.uv.pipe').new()
    local _, err = M.client:connect(path):get()

    if not err then
      self.status = 'connected'
      logger.debug 'Connected to pipe'

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
    M.tx = require('cord.server.event.sender').new(M.client)
    M.rx = require('cord.server.event.receiver').new(M.client)
    logger.debug 'Server: sending initialize event'
    M.tx:initialize(config.get())
    logger.debug 'Server: registering ready handler'
    M.rx:register(
      'ready',
      true,
      vim.schedule_wrap(function()
        self.status = 'ready'
        async.run(function()
          logger.info 'Connected to Discord'

          local ActivityManager = require 'cord.plugin.activity.manager'
          local manager, err = ActivityManager.new({ tx = M.tx }):get()
          if not manager or err then
            self.status = 'disconnected'
            self.client:close()
            logger.error(err or 'Failed to initialize activity manager')
            return
          end

          M.client.on_close = vim.schedule_wrap(function()
            M.status = 'disconnected'
            if M.manager then M.manager:cleanup() end
            require('cord.plugin.activity.hooks').run 'shutdown'
          end)

          manager:run()
          M.manager = manager

          M.rx:register(
            'disconnect',
            false,
            vim.schedule_wrap(function()
              self.status = 'connected'
              M.manager:cleanup()
              require('cord.plugin.activity.hooks').run 'shutdown'

              if config.advanced.discord.reconnect.enabled then logger.info 'Reconnecting...' end

              M.rx:register(
                'ready',
                true,
                vim.schedule_wrap(function()
                  self.status = 'ready'
                  logger.info 'Connected to Discord'
                  M.manager:run()
                end)
              )
            end)
          )
        end)
      end)
    )

    logger.debug 'Server initialized; starting receiver'
    M.rx:run()
  end)()
end

function M:initialize()
  self.status = 'connecting'
  async.run(function()
    logger.debug 'Initializing server...'

    local path = config.advanced.server.pipe_path
      or require('cord.plugin.constants').get_pipe_path()

    logger.trace(function() return 'Server pipe path: ' .. tostring(path) end)
    local _, err = M:connect(path):get()
    if err then
      self.status = 'disconnected'
      logger.error(err)
    end
  end)
end

function M:cleanup()
  if self.manager then self.manager:cleanup() end
  if self.client then self.client:close() end
end

return M
