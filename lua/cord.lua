local config = require 'cord.util.config'
local ipc = require 'cord.core.ipc'
local logger = require 'cord.util.logger'

local M = {}

function M.setup(opts)
  config:validate(opts or {})
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

      M.manager:run_event_loop()
    end)

    M.handler:run()
  end)

  M.client = client
end

return M
