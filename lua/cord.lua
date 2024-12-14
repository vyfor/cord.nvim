local config = require 'cord.util.config'
local ipc = require 'cord.core.ipc'
local logger = require 'cord.util.logger'

local M = {}

function M.initialize()
  local client = ipc.new(config.values)
  client:connect(function()
    local file_manager = require 'cord.util.file_manager'
    local Producer = require 'cord.event.sender'
    local Handler = require 'cord.event.receiver'
    local uv = vim.loop or vim.uv

    M.producer = Producer.new(client)
    M.handler = Handler.new(client)
    M.handler:register('initialize', function(pid)
      local executable = file_manager.get_executable_name()
      local target_path = file_manager.get_target_path(executable)
      uv.fs_stat(target_path, function(err)
        if not err then
          client:on_close(function()
            file_manager.get_executable(pid, function(_, err, moved)
              if err then
                logger.error(err)
                return
              end

              if moved then
                client:close()
                M.initialize()
              end
            end)
          end)
          M.producer:shutdown()
        else
          M.handler:register(
            'ready',
            vim.schedule_wrap(function()
              logger.info 'Connected to Discord'

              local ActivityManager = require 'cord.activity.manager'

              if config.values.hooks.on_ready then
                config.values.hooks.on_ready()
              end

              M.producer:initialize(config.values)

              ActivityManager.new(
                { tx = M.producer, config = config.values },
                vim.schedule_wrap(function(manager)
                  M.manager = manager

                  client:on_close(vim.schedule_wrap(function()
                    if config.values.hooks.on_disconnect then
                      config.values.hooks.on_disconnect()
                    end

                    manager:pause()
                  end))
                  manager:run()
                end)
              )
            end)
          )
        end
      end)
    end)

    M.handler:run()
  end)
end

---@param opts? CordConfig Configuration options for Cord
---@return nil
function M.setup(opts)
  if not config:validate(opts or {}) then return end
  logger.set_level(config.values.log_level)
  M.initialize()
end

return M
