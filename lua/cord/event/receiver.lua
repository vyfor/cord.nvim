local mpack = vim.mpack
local logger = require 'cord.util.logger'
local bit = require 'bit'

local Handler = {}
local mt = { __index = Handler }

function Handler.new(client)
  local self = setmetatable({}, mt)
  self.client = client
  self.handlers = {}
  self.queue = {}
  return self
end

function Handler:on_event(type, data)
  local handler = self.handlers[type]
  if handler then
    handler(data)
  else
    self.queue[type] = data
  end
end

function Handler:register(type, callback)
  local data = self.queue[type]
  if data then
    callback(data)
    self.queue[type] = nil
  end

  self.handlers[type] = callback
end

function Handler:run()
  self:setup_default_handlers()

  local buffer = ''

  self.client:read_start(function(data)
    if not data then return end

    buffer = buffer .. data

    while #buffer >= 4 do
      local length = bit.lshift(string.byte(buffer, 1), 24)
        + bit.lshift(string.byte(buffer, 2), 16)
        + bit.lshift(string.byte(buffer, 3), 8)
        + string.byte(buffer, 4)

      if #buffer < 4 + length then break end

      local message = string.sub(buffer, 5, 4 + length)
      buffer = string.sub(buffer, 5 + length)

      local ok, event = pcall(mpack.decode, message)
      if not ok then
        logger.error(
          'Failed to decode event: ' .. event .. '; data: ' .. message
        )
        goto continue
      end

      if not event or not event.type then
        logger.error 'Invalid event format'
        goto continue
      end

      self:on_event(event.type, event.data)

      ::continue::
    end
  end)
end

function Handler:setup_default_handlers()
  self:register('log', function(data)
    if data.level and data.message then
      logger.log_raw(data.level, data.message)
      if data.level == vim.log.levels.ERROR then require('cord').cleanup() end
    end
  end)
end

return Handler
