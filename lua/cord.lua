local config = require 'cord.util.config'
local ipc = require 'cord.core.ipc'
local logger = require 'cord.util.logger'

local M = {}

function M.initialize()
  M.client = ipc.new(config.values)
  M.client:connect(function()
    local file_manager = require 'cord.util.file_manager'
    local Producer = require 'cord.event.sender'
    local Handler = require 'cord.event.receiver'
    local uv = vim.loop or vim.uv

    M.producer = Producer.new(M.client)
    M.handler = Handler.new(M.client)
    M.handler:register('initialize', function(pid)
      vim.g.cord_pid = pid
      local executable = file_manager.get_executable_name()
      local target_path = file_manager.get_target_path(executable)
      uv.fs_stat(target_path, function(err)
        if not err then
          M.client:on_close(function()
            file_manager.get_executable(
              pid,
              vim.schedule_wrap(function(_, err, moved)
                if err then
                  logger.error(err)
                  return
                end

                if moved then
                  M.client:close()
                  M.initialize()
                end
              end)
            )
          end)
          M.producer:shutdown()
        else
          M.handler:register(
            'ready',
            vim.schedule_wrap(function()
              logger.info 'Connected to Discord'

              local ActivityManager = require 'cord.activity.manager'

              M.producer:initialize(config.values)

              M.manager = ActivityManager.new(
                { tx = M.producer, config = config.values },
                vim.schedule_wrap(function(manager)
                  M.client:on_close(vim.schedule_wrap(function()
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
  if config:validate(opts or {}) then M.initialize() end
end

function M.cleanup()
  if M.manager then
    M.manager.clear_autocmds()
    M.manager.idle_timer:close()
    M.manager = nil
  end
  if M.client then
    M.client:close()
    M.client = nil
  end
  M.producer = nil
  M.handler = nil
end

return M
