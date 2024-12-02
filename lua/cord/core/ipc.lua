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
  local path = utils.os_name == 'Windows' and '\\\\.\\pipe\\' or '/tmp/'
  local pipe = uv.new_pipe()
  self.pipe = pipe

  pipe:connect(
    path .. self.config.advanced.server.pipe_name,
    vim.schedule_wrap(function(err)
      if err then
        if err == 'ENOENT' then
          local executable = self.config.advanced.server.executable_path

          if not utils.file_exists(executable) then
            logger.error(
              'Server executable not found at \'' .. executable .. '\''
            )
            return
          end

          local stdout = uv.new_pipe()
          local stderr = uv.new_pipe()

          uv.spawn(
            executable,
            {
              args = {
                '-p',
                self.config.advanced.server.pipe_name,
                '-c',
                self.config.editor.client,
                '-t',
                tostring(self.config.advanced.server.timeout),
              },
              stdio = { nil, stdout, stderr },
              detached = true,
              hide = true,
            },
            vim.schedule_wrap(function(code, _)
              if code ~= 0 then
                logger.error('Failed to start server: exit code ' .. code)
                return
              end
            end)
          )

          stderr:read_start(vim.schedule_wrap(function(err, chunk)
            if err then
              logger.error('Failed to read stderr: ' .. err)
              return
            end
            if chunk then logger.error('Server error: ' .. chunk) end
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

      logger.debug(
        'Connected to pipe: ' .. self.config.advanced.server.pipe_name
      )

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
