local mpack = vim.mpack
local logger = require 'cord.api.log'

local Producer = {}
local mt = { __index = Producer }

function Producer.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  logger.trace 'Sender.new: client created'
  return self
end

function Producer:send_event(type, data)
  logger.trace(function() return 'Sender:send_event: type=' .. tostring(type) end)
  if self.client:is_closing() then return end
  self.client:write(mpack.encode { type = type, data = data })
end

function Producer:initialize(config)
  logger.debug 'Sender:initialize called'
  self:send_event('initialize', {
    log_level = config.log_level,
    timestamp = {
      shared = config.timestamp.shared,
    },
    advanced = {
      discord = {
        pipe_paths = config.advanced.discord.pipe_paths,
        sync = {
          enabled = config.advanced.discord.sync.enabled,
          mode = config.advanced.discord.sync.mode,
          interval = config.advanced.discord.sync.interval,
          reset_on_update = config.advanced.discord.sync.reset_on_update,
          pad = config.advanced.discord.sync.pad,
        },
      },
    },
  })
end

function Producer:update_activity(activity, force)
  self:send_event('update_activity', { activity = activity, force = force })
end

function Producer:clear_activity(force) self:send_event('clear_activity', force or false) end

function Producer:disconnect() self:send_event 'disconnect' end

function Producer:shutdown() self:send_event 'shutdown' end

function Producer:restart() self:send_event 'restart' end

return Producer
