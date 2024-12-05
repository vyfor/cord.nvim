local config = require 'cord.util.config'
local ipc = require 'cord.core.ipc'
local logger = require 'cord.util.logger'

local M = {}

function M.setup(opts)
  if not config:validate(opts or {}) then return end
  logger.set_level(config.values.log_level)

  local client = ipc.new(config.values)
  client:connect(function()
    local Handler = require 'cord.event.receiver'
    M.handler = Handler.new(client)

    M.handler:register('ready', function()
      logger.info 'Connected to Discord'

      local Producer = require 'cord.event.sender'
      local ActivityManager = require 'cord.activity.manager'

      if config.values.hooks.on_ready then config.values.hooks.on_ready() end

      M.producer = Producer.new(client)
      M.producer:initialize(config.values)
      M.manager =
        ActivityManager.new { tx = M.producer, config = config.values }

      client:on_close(function()
        if config.values.hooks.on_disconnect then
          config.values.hooks.on_disconnect()
        end

        M.manager:pause()
      end)

      M.manager:run()
    end)

    M.handler:run()
  end)

  M.client = client
end

return M
