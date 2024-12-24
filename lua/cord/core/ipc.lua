local logger = require 'cord.util.logger'
local utils = require 'cord.util'
local spawn = require 'cord.core.spawn'

local uv = vim.loop or vim.uv

local IPC = {}
local mt = { __index = IPC }

function IPC.new(config)
  local self = setmetatable({}, mt)
  self.config = config
  self.pipe = nil
  return self
end

local function kill_process(callback)
  if utils.os_name == 'windows' then
    uv.spawn('taskkill', {
      args = {
        '/F',
        '/IM',
        'cord.exe',
      },
    }, function() callback() end)
  else
    uv.spawn('sh', {
      args = {
        '-c',
        'pkill -15 -x cord',
      },
    }, function() callback() end)
  end
end

function IPC:connect(callback, attempts, err)
  if not attempts then attempts = 0 end
  if attempts > 5 then
    logger.error('Failed to connect to pipe: ' .. (err or 'nil'))
  end

  if self.config.advanced.server.pipe_path then
    self.path = self.config.advanced.server.pipe_path
  else
    self.path = (utils.os_name == 'windows' and '\\\\.\\pipe\\' or '/tmp/')
      .. 'cord-ipc'
  end
  local pipe = uv.new_pipe()
  self.pipe = pipe

  pipe:connect(self.path, function(err)
    if err then
      if err == 'ENOENT' then
        logger.debug 'Pipe not found, spawning server...'

        spawn.spawn_server(self, function() self:connect(callback) end)
        return
      elseif err == 'ECONNREFUSED' then
        logger.debug 'Received ECONNREFUSED, retrying...'

        if attempts == 3 then
          kill_process(function()
            spawn.spawn_server(self, function() self:connect(callback) end)
          end)
        else
          vim.defer_fn(
            function() self:connect(callback, attempts + 1, err) end,
            1000
          )
        end
      elseif err == 'ETIMEDOUT' then
        logger.debug 'Received ETIMEDOUT, retrying...'

        if attempts == 3 then
          kill_process(function()
            spawn.spawn_server(self, function() self:connect(callback) end)
          end)
        else
          vim.defer_fn(
            function() self:connect(callback, attempts + 1, err) end,
            1000
          )
        end
      else
        logger.error('Failed to connect to pipe: ' .. err)
      end
      return
    end

    logger.debug('Connected to pipe: ' .. self.path)

    if callback then callback() end
  end)
end

function IPC:read_start(callback)
  if not self.pipe then return end

  self.pipe:read_start(function(err, chunk)
    if err then
      logger.error('Read error: ' .. err)
      return
    end

    if chunk then
      if callback then callback(chunk) end
    else
      self:close()
    end
  end)
end

function IPC:write(data, callback)
  if not self.pipe then return false end

  self.pipe:write(data, function(err)
    if err then
      logger.error('Write error: ' .. err)
      return
    end

    if callback then callback() end
  end)

  return true
end

function IPC:on_close(callback) self.on_close_cb = callback end

function IPC:close()
  if self.pipe then
    logger.debug 'Connection closed'
    self.pipe:read_stop()
    self.pipe:close()
    self.pipe = nil

    if self.on_close_cb then self.on_close_cb() end
  end
end

return IPC
