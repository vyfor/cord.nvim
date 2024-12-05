local mpack = vim.mpack

local Producer = {}
local mt = { __index = Producer }

function Producer.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  return self
end

function Producer:send_event(type, data)
  self.client:write(mpack.encode { type = type, data = data })
end

function Producer:initialize(config)
  local config = {
    log_level = config.log_level,
  }

  self:send_event('initialize', config)
end

function Producer:update_activity(activity)
  self:send_event('update_activity', activity)
end

function Producer:clear_activity(force)
  self:send_event('clear_activity', force or false)
end

function Producer:disconnect() self:send_event 'disconnect' end

return Producer
