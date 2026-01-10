local logger = require 'cord.api.log'
local bit = require 'bit'

local Handler = {}
local mt = { __index = Handler }

function Handler.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  self.handlers = {}
  self.queue = {}
  logger.trace 'Receiver.new: client created'
  return self
end

function Handler:on_event(type, data)
  logger.trace(function() return 'Receiver:on_event: type=' .. tostring(type) end)
  local handler = self.handlers[type]
  if handler then
    if handler.oneshot then self.handlers[type] = nil end
    handler.callback(data)
  else
    self.queue[type] = data
  end
end

function Handler:register(type, oneshot, callback)
  local data = self.queue[type]
  if data then
    logger.trace(
      function()
        return 'Receiver:register immediate dispatch: type='
            .. tostring(type)
            .. ', oneshot='
            .. tostring(oneshot)
      end
    )
    callback(data)
    self.queue[type] = nil
    if oneshot then return end
  end

  self.handlers[type] = {
    oneshot = oneshot,
    callback = callback,
  }
  logger.trace(
    function()
      return 'Receiver:registered handler: type='
          .. tostring(type)
          .. ', oneshot='
          .. tostring(oneshot)
    end
  )
end

function Handler:run()
  logger.debug 'Receiver: starting event loop'
  self:setup_default_handlers()

  local buffer = ''
  self.client:read_start(function(data)
    buffer = buffer .. data

    while #buffer >= 4 do
      local length = bit.lshift(string.byte(buffer, 1), 24)
          + bit.lshift(string.byte(buffer, 2), 16)
          + bit.lshift(string.byte(buffer, 3), 8)
          + string.byte(buffer, 4)

      if #buffer < 4 + length then break end

      local message = string.sub(buffer, 5, 4 + length)
      buffer = string.sub(buffer, 5 + length)

      local ok, event = pcall(vim.mpack.decode, message)
      if not ok then
        logger.error('Failed to decode event: ' .. event .. '; data: ' .. message)
        goto continue
      end

      if not event or not event.type then
        logger.error 'Invalid event format'
        goto continue
      end

      logger.trace(function() return 'Receiver:decoded event: type=' .. tostring(event.type) end)
      self:on_event(event.type, event.data)

      ::continue::
    end
  end)
end

function Handler:setup_default_handlers()
  self:register('log', false, function(data)
    if data.level and data.message then
      logger.log(data.message, data.level)
    end
  end)
  logger.trace 'Receiver: default handlers registered'
end

return Handler
