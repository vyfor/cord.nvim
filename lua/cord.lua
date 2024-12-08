local config = require 'cord.util.config'
local ipc = require 'cord.core.ipc'
local logger = require 'cord.util.logger'

local M = {}

function M.initialize(config)
  local client = ipc.new(config.values)
  client:connect(function()
    local file_manager = require 'cord.util.file_manager'
    local utils = require 'cord.util'
    local Producer = require 'cord.event.sender'
    local Handler = require 'cord.event.receiver'

    M.producer = Producer.new(client)
    M.handler = Handler.new(client)

    local executable = file_manager.get_executable_name()
    local target_path = file_manager.get_target_path(executable)
    if utils.file_exists(target_path) then
      M.handler:register('shutdown', function()
        local data_path = file_manager.get_data_path()
        local executable_path = data_path .. utils.path_sep .. executable

        if not utils.file_exists(executable_path) then
          utils.mkdir(data_path)
        else
          local ok, err = utils.rm_file(executable_path)
          if not ok then
            logger.error(
              'Failed to remove existing executable: ' .. (err or '')
            )
            return
          end
        end

        local ok, err = utils.move_file(target_path, executable_path)
        if not ok then
          logger.error('Failed to move executable: ' .. (err or ''))
          return
        end

        client:close()
        M.initialize(config)
      end)
      M.producer:shutdown()
    else
      M.handler:register('ready', function()
        logger.info 'Connected to Discord'

        local ActivityManager = require 'cord.activity.manager'

        if config.values.hooks.on_ready then config.values.hooks.on_ready() end

        M.producer:initialize(config.values)
        M.manager =
          ActivityManager.new { tx = M.producer, config = config.values }

        client:on_close(function()
          if config.values.hooks.on_disconnect then
            config.values.hooks.on_disconnect()
          end

          M.manager:pause()
        end)

        M.manager:run()
      end)
    end

    M.handler:run()
  end)
end

function M.setup(opts)
  if not config:validate(opts or {}) then return end
  logger.set_level(config.values.log_level)
  M.initialize(config)
end

return M
