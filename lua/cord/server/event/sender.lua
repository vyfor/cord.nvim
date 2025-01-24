local mpack = vim.mpack

local Producer = {}
local mt = { __index = Producer }

function Producer.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  return self
end

function Producer:send_event(type, data)
  if self.client:is_closing() then return end
  self.client:write(mpack.encode { type = type, data = data })
end

function Producer:initialize(config)
  self:send_event(
    'initialize',
    { log_level = config.log_level }
  )
end

function Producer:update_activity(activity)
  self:send_event('update_activity', activity)
end

function Producer:clear_activity(force)
  self:send_event('clear_activity', force or false)
end

function Producer:disconnect() self:send_event 'disconnect' end

function Producer:shutdown() self:send_event 'shutdown' end

return Producer
