local ipc = require 'cord.core.ipc'

local M = {}

function M.setup(opts)
  local client = ipc.new { pipe_name = '\\\\.\\pipe\\cord-ipc' }

  client:connect(function()
    local Handler = require 'cord.event.receiver'

    M.handler = Handler.new(client)

    M.handler:register('ready', function()
      local config = require 'cord.util.config'
      local logger = require 'cord.util.logger'

      config:validate(opts or {})
      logger.set_level(config.values.log_level)
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
