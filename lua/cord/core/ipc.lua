local uv = vim.loop
local logger = require 'cord.util.logger'
local utils = require 'cord.util'

local IPC = {}
local mt = { __index = IPC }

function IPC.new(config)
  local self = setmetatable({}, mt)
  self.config = config
  self.pipe = nil
  return self
end

function IPC:connect(callback)
  if self.config.advanced.server.pipe_path then
    self.path = self.config.advanced.server.pipe_path
  else
    self.path = (utils.os_name == 'Windows' and '\\\\.\\pipe\\' or '/tmp/')
      .. 'cord-ipc'
  end
  local pipe = uv.new_pipe()
  self.pipe = pipe

  pipe:connect(
    self.path,
    vim.schedule_wrap(function(err)
      if err then
        if err == 'ENOENT' then
          if self.config.advanced.server.executable_path then
            self.executable = self.config.advanced.server.executable_path
          else
            self.executable = utils.os_name == 'Windows'
                and 'target/release/cord.exe'
              or 'target/release/cord'
          end

          if not utils.file_exists(self.executable) then
            logger.error(
              'Server executable not found at \'' .. self.executable .. '\''
            )
            return
          end

          local stdout = uv.new_pipe()
          local stderr = uv.new_pipe()
          uv.spawn(self.executable, {
            args = {
              '-p',
              self.path,
              '-c',
              self.config.editor.client,
              '-t',
              tostring(self.config.advanced.server.timeout),
            },
            stdio = { nil, stdout, stderr },
            detached = true,
            hide = true,
          })

          stderr:read_start(vim.schedule_wrap(function(err, chunk)
            if err then
              logger.error('Failed to read stderr: ' .. err)
              return
            end
            if chunk then
              if chunk:match 'kind: AlreadyExists' then
                self:connect(callback)
                stderr:close()
                stdout:close()
                return
              end
              logger.error('Server error: ' .. chunk)
            end
          end))

          stdout:read_start(vim.schedule_wrap(function(err, chunk)
            if err then
              logger.error('Failed to read pipe: ' .. err)
              return
            end

            if chunk and chunk:match 'Ready' then
              self:connect(callback)
              return
            end
          end))

          return
        else
          logger.error('Failed to connect to pipe: ' .. err)
        end
        return
      end

      logger.debug('Connected to pipe: ' .. self.path)

      if callback then callback() end
    end)
  )
end

function IPC:read_start(callback)
  if not self.pipe then return end

  self.pipe:read_start(vim.schedule_wrap(function(err, chunk)
    if err then
      logger.error('Read error: ' .. err)
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
