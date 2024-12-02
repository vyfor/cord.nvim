local uv = vim.loop
local logger = require 'cord.util.logger'

local IPC = {}
local mt = { __index = IPC }

function IPC.new(opts)
  local self = setmetatable({}, mt)
  self.pipe_name = opts.pipe_name
  self.config = opts.config
  self.pipe = nil
  return self
end

function IPC:connect(callback)
  local pipe = uv.new_pipe()
  self.pipe = pipe

  pipe:connect(
    self.pipe_name,
    vim.schedule_wrap(function(err)
      if err then
        if err == 'ENOENT' then
          logger.error('Pipe not found: ' .. self.pipe_name)
        else
          logger.error('Failed to connect to pipe: ' .. err)
        end
        self:on_error(err)
        return
      end

      logger.debug('Connected to pipe: ' .. self.pipe_name)

      if callback then callback() end
    end)
  )
end

function IPC:read_start(callback)
  if not self.pipe then return end

  self.pipe:read_start(vim.schedule_wrap(function(err, chunk)
    if err then
      logger.error('Read error: ' .. err)
      self.on_error(err)
      return
    end

    if chunk then
      if callback then callback(chunk) end
    else
      self:close()
    end
  end))
end

function IPC:write(data, callback)
  if not self.pipe then return false end

  self.pipe:write(
    data,
    vim.schedule_wrap(function(err)
      if err then
        logger.error('Write error: ' .. err)
        self:on_error(err)
        return
      end

      if callback then callback() end
    end)
  )

  return true
end

function IPC:close()
  if self.pipe then
    logger.debug 'Connection closed'
    self.pipe:read_stop()
    self.pipe:close()
    self.pipe = nil
  end
end

return IPC
