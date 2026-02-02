local async = require 'cord.core.async'
local logger = require 'cord.api.log'

local M = {}

local function parse_server_version(content)
  if not content then return nil end
  local trimmed = content:gsub('^%s*(.-)%s*$', '%1')
  if trimmed == '' then return nil end

  local version_part, timestamp_part = trimmed:match '^([^|]+)|(.+)$'
  if version_part then
    local timestamp = tonumber(timestamp_part)
    return version_part, timestamp
  end

  return trimmed, nil
end

local request_shutdown = async.wrap(function(pipe_path)
  if not pipe_path or pipe_path == '' then return false end

  local client = require('cord.core.uv.pipe').new()
  local _, err = client:connect(pipe_path):await()
  if err then
    client:close()
    return false
  end

  local tx = require('cord.server.ipc.sender').new(client)
  tx:shutdown()
  client:close()
  return true
end)

M.install = async.wrap(function()
  local server = require 'cord.server'
  if server.is_updating then return end
  server.is_updating = true

  if not vim.fn.executable 'cargo' then
    error 'cargo is not installed or not in PATH'
    return
  end

  logger.info 'Building executable...'

  vim.schedule(function()
    local cord = require 'cord.server'
    local function initialize()
      local process = require 'cord.core.uv.process'

      async.run(function()
        process
          .spawn({
            cmd = 'cargo',
            args = {
              'install',
              'cord-nvim',
              '--force',
              '--root',
              require('cord.server.fs').get_data_path(),
            },
          })
          :next(function(res)
            if res.code ~= 0 then
              server.is_updating = false
              logger.error 'Failed to build executable'
              if res.stderr then logger.error('cargo\'s stderr: ' .. res.stderr) end
              return
            end
            logger.notify('Successfully built executable. Restarting...', vim.log.levels.INFO)

            async.run(function()
              server.is_updating = false
              cord:initialize()
            end)
          end, function(err)
            server.is_updating = false

            logger.error(err)
          end)
      end)
    end

    if cord.manager then cord.manager:cleanup() end
    if not cord.tx then return initialize() end
    if not cord.client then return initialize() end
    if cord.client:is_closing() then return initialize() end

    if cord.client.on_close then cord.client.on_close() end

    cord.client.on_close = function()
      cord.client.on_close = nil
      initialize()
    end

    cord.tx:shutdown()
  end)
end)

M.build = async.wrap(function()
  local server = require 'cord.server'
  if server.is_updating then return end
  server.is_updating = true

  if not vim.fn.executable 'cargo' then
    error 'cargo is not installed or not in PATH'
    return
  end

  logger.info 'Building executable locally...'

  vim.schedule(function()
    local cord = require 'cord.server'
    local function initialize()
      local process = require 'cord.core.uv.process'

      async.run(function()
        process
          .spawn({
            cmd = 'cargo',
            args = {
              'install',
              '--path',
              require('cord.server.fs').get_plugin_root(),
              '--force',
              '--root',
              require('cord.server.fs').get_data_path(),
            },
          })
          :next(function(res)
            if res.code ~= 0 then
              server.is_updating = false
              logger.error 'Failed to build executable'
              if res.stderr then logger.error('cargo\'s stderr: ' .. res.stderr) end
              return
            end
            logger.notify('Successfully built executable. Restarting...', vim.log.levels.INFO)

            async.run(function()
              server.is_updating = false
              cord:initialize()
            end)
          end, function(err)
            server.is_updating = false

            logger.error(err)
          end)
      end)
    end

    if cord.manager then cord.manager:cleanup() end
    if not cord.tx then return initialize() end
    if not cord.client then return initialize() end
    if cord.client:is_closing() then return initialize() end

    if cord.client.on_close then cord.client.on_close() end

    cord.client.on_close = function()
      cord.client.on_close = nil
      initialize()
    end

    cord.tx:shutdown()
  end)
end)

M.local_version = async.wrap(function()
  local fs = require 'cord.core.uv.fs'
  local process = require 'cord.core.uv.process'
  local executable_path =
    require('cord.server.fs').get_executable_path(require('cord.api.config').get())

  if not fs.stat(executable_path):await() then return nil end

  local res = process
    .spawn({
      cmd = executable_path,
      args = { '-v' },
    })
    :await()

  if not res then return nil end
  if res.code ~= 0 then return nil end
  local version = res.stdout:gsub('^%s*(.-)%s*$', '%1')
  if not version then return nil end

  return version
end)

M.compatible_version = async.wrap(function()
  local metadata = M.compatible_metadata():unwrap()
  return metadata.version
end)

M.compatible_metadata = async.wrap(function()
  local fs = require 'cord.core.uv.fs'
  local path = require('cord.server.fs').get_plugin_root() .. '/.github/server-metadata.txt'

  local content = fs.readfile(path):await()
  if not content then return nil end
  local version, timestamp = parse_server_version(content)
  if not version then return nil end

  return { version = version, timestamp = timestamp }
end)

M.remote_version = async.wrap(function()
  local metadata = M.remote_metadata():unwrap()
  return metadata.version
end)

M.remote_metadata = async.wrap(function()
  local process = require 'cord.core.uv.process'
  local res = process
    .spawn({
      cmd = 'curl',
      args = {
        'https://raw.githubusercontent.com/vyfor/cord.nvim/refs/heads/master/.github/server-metadata.txt',
        '--fail',
        '--silent',
        '--show-error',
      },
    })
    :unwrap()

  if res.code ~= 0 then
    error('Failed to fetch latest version; code: ' .. tostring(res.code), 0)
    if res.stderr and res.stderr ~= '' then logger.debug('curl stderr: ' .. res.stderr) end
    return nil
  end

  local version, timestamp = parse_server_version(res.stdout)
  if not version or version == '' then
    error('Failed to parse latest version', 0)
    return nil
  end

  return { version = version, timestamp = timestamp }
end)

M.is_stale = async.wrap(function(local_mtime)
  local metadata = M.compatible_metadata():unwrap()
  if not metadata.version or not metadata.timestamp then return false end

  local local_seconds = local_mtime
  if local_mtime then local_seconds = local_mtime.sec end

  if not local_seconds then return false end
  return local_seconds < metadata.timestamp
end)

M.check_version = async.wrap(function()
  local server = require 'cord.server'
  if server.is_updating then return end

  if not vim.fn.executable 'curl' then
    error('curl is not installed or not in PATH', 0)
    return
  end

  async.run(function()
    logger.notify('Checking for updates...', vim.log.levels.INFO)

    local current = M.local_version():unwrap()
    local compatible = M.compatible_version():unwrap()
    local latest = M.remote_version():unwrap()

    logger.debug(
      'current: '
        .. tostring(current)
        .. '; compatible: '
        .. tostring(compatible)
        .. '; latest: '
        .. tostring(latest)
    )

    if not current then
      logger.notify(
        'Server executable is missing. Please run `:Cord update` to install it.',
        vim.log.levels.WARN
      )
    elseif compatible and compatible ~= current then
      logger.notify(
        'The local server version ('
          .. current
          .. ') does not match the latest compatible version ('
          .. compatible
          .. '). Please run `:Cord update`',
        vim.log.levels.WARN
      )
    end

    if compatible and latest then
      if latest == compatible then
        if current == compatible then
          logger.notify('You are on the latest version ' .. compatible, vim.log.levels.INFO)
        end
      else
        logger.notify(
          'New version available: '
            .. latest
            .. ' (current compatible: '
            .. compatible
            .. '). Please update the plugin.',
          vim.log.levels.INFO
        )
      end
    end
  end)
end)

M.version = async.wrap(function()
  async.run(function()
    local version = M.local_version():unwrap()
    if version then logger.notify('Server version: ' .. version, vim.log.levels.INFO) end
  end)
end)

M.fetch_executable = async.wrap(function(tag, pipe_path)
  local server = require 'cord.server'
  if server.is_updating then return end
  server.is_updating = true

  if not vim.fn.executable 'curl' then
    error('curl is not installed or not in PATH', 0)
    return
  end

  local executable_path =
    require('cord.server.fs').get_executable_path(require('cord.api.config').get())
  local process = require 'cord.core.uv.process'

  local schedule_fetch = vim.schedule_wrap(function(tag)
    local base_url
    if tag then
      logger.info('Downloading version ' .. tag .. '...')
      base_url = 'https://github.com/vyfor/cord.nvim/releases/download/v' .. tag .. '/'
    else
      logger.info 'Downloading latest version...'
      base_url = 'https://github.com/vyfor/cord.nvim/releases/latest/download/'
    end

    local os_info = require('cord.core.util').get_os()
    local url = base_url
      .. os_info.arch
      .. '-'
      .. os_info.name
      .. '-'
      .. (os_info.name == 'windows' and 'cord.exe' or 'cord')

    local function initialize()
      async.run(function()
        if pipe_path then
          logger.debug 'Shutting down existing server...'
          request_shutdown(pipe_path):unwrap()
        end

        process
          .spawn({
            cmd = 'curl',
            args = {
              url,
              '--create-dirs',
              '--fail',
              '--location',
              '--remote-time',
              '--silent',
              '--show-error',
              '-o',
              executable_path,
              '-H',
              'Accept: application/octet-stream',
            },
          })
          :next(function(res)
            if res.code ~= 0 then
              server.is_updating = false
              logger.error('Failed to download executable; code: ' .. res.code .. ', path: ' .. url)
              if res.stderr and res.stderr ~= '' then
                logger.error('curl\'s stderr: ' .. res.stderr)
              end
              return
            end
            logger.notify('Successfully updated executable. Restarting...', vim.log.levels.INFO)

            async.run(function()
              server.is_updating = false
              require('cord.core.uv.fs').chmod(executable_path, '755'):unwrap()
              server:initialize()
            end)
          end, function(err)
            server.is_updating = false
            logger.error(err)
          end)
      end)
    end

    if server.manager then server.manager:cleanup() end
    if server.tx and server.client and not server.client:is_closing() then
      if server.client.on_close then server.client.on_close() end

      server.client.on_close = function()
        server.client.on_close = nil
        initialize()
      end

      server.tx:shutdown()
      return
    end

    initialize()
  end)

  schedule_fetch(tag)
end)

M.fetch = async.wrap(function()
  local server = require 'cord.server'
  if server.is_updating then return end

  async.run(function()
    logger.info 'Checking for updates...'
    local current = M.local_version():unwrap()
    local compatible = M.compatible_version():unwrap()

    if compatible then
      if current ~= compatible then
        M.fetch_executable(compatible):unwrap()
      else
        server.is_updating = false
        logger.info('Already on latest compatible server version ' .. current)

        if not server.client or server.client:is_closing() then server:initialize() end
      end
    else
      logger.warn 'Could not determine compatible server version. Fetching latest...'

      local latest = M.remote_version():unwrap()
      if latest and current == latest then
        server.is_updating = false
        logger.info('Already on latest server version ' .. current)

        if not server.client or server.client:is_closing() then server:initialize() end
      else
        M.fetch_executable(latest):unwrap()
      end
    end
  end)
end)

M.auto_update = async.wrap(function(config, pipe_path)
  local server = require 'cord.server'
  if server.is_updating then return false end

  local update_strategy = config.advanced.server.update
  if update_strategy ~= 'fetch' then return false end
  if config.advanced.server.auto_update == false then return false end

  local exec_path = require('cord.server.fs').get_executable_path(config)
  local fs = require 'cord.core.uv.fs'
  local stat = fs.stat(exec_path):await()

  if not stat then return false end

  local is_stale = M.is_stale(stat.mtime):unwrap()
  if not is_stale then return false end

  local compatible = M.compatible_version():unwrap()
  if compatible then
    M.fetch_executable(compatible, pipe_path):unwrap()
    return true
  end

  return false
end)

return M
