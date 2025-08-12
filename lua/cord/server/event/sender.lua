local mpack = vim.mpack
local logger = require 'cord.plugin.log'

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
  })
end

function Producer:update_activity(activity) self:send_event('update_activity', activity) end

function Producer:clear_activity(force) self:send_event('clear_activity', force or false) end

function Producer:disconnect() self:send_event 'disconnect' end

function Producer:shutdown() self:send_event 'shutdown' end

return Producer
