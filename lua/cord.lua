local logger = require 'cord.util.logger'
local ipc = require 'cord.core.ipc'
local config = require 'cord.util.config'
local utils = require 'cord.util'
local Handler = require 'cord.event.receiver'
local Producer = require 'cord.event.sender'
local ActivityManager = require 'cord.activity.manager'
local workspace = require 'cord.util.workspace'

local M = {}

function M.setup(opts)
  config:validate(opts or {})
  logger.set_level(config.values.log_level)

  local client = ipc.new {
    pipe_name = '\\\\.\\pipe\\cord-ipc',
  }
  local handler = Handler.new(client)

  handler:register('ready', function()
    logger.info 'Connected to Discord'

    if config.values.hooks.on_ready then config.values.hooks.on_ready() end

    M.producer = Producer.new(client)
    M.producer:initialize(config.values)
    M.manager = ActivityManager.new { tx = M.producer, config = config.values }
    M.manager:run_event_loop()
  end)

  client:connect(function() handler:run() end)

  M.client = client
  M.handler = handler
end

return {
  setup = M.setup,
}
