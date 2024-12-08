local mpack = vim.mpack
local logger = require 'cord.util.logger'

local Handler = {}
local mt = { __index = Handler }

function Handler.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  self.handlers = {}
  return self
end

function Handler:on_event(type, data)
  local handler = self.handlers[type]
  if handler then handler(data) end
end

function Handler:register(type, callback) self.handlers[type] = callback end

function Handler:run()
  self:setup_default_handlers()
  self.client:read_start(function(data, err)
    if err then
      logger.error('Error handling event: ' .. err)
      self.client:close()
      return
    end

    if not data then return end

    local ok, event = pcall(mpack.decode, data)
    if not ok then
      logger.error('Failed to decode event: ' .. event)
      return
    end

    if not event or not event.type then
      logger.error 'Invalid event format'
      return
    end

    self:on_event(event.type, event.data)
  end)
end

function Handler:setup_default_handlers()
  self:register('log', function(data)
    if data.level and data.message then
      logger.log_raw(data.message, data.level)
    end
  end)
end

return Handler
