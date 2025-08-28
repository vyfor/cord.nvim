local async = require 'cord.core.async'
local logger = require 'cord.api.log'

local M = {}

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
            :and_then(function(res)
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
            :and_then(function(res)
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
  local process = require 'cord.core.uv.process'
  local executable_path =
      require('cord.server.fs').get_executable_path(require('cord.api.config').get())

  local res = process
      .spawn({
        cmd = executable_path,
        args = { '-v' },
      })
      :get()

  if not res then return nil end
  if res.code ~= 0 then return nil end
  local version = res.stdout:gsub('^%s*(.-)%s*$', '%1')
  if not version then return nil end

  return version
end)

M.remote_version = async.wrap(function()
  local process = require 'cord.core.uv.process'
  local res = process
      .spawn({
        cmd = 'curl',
        args = {
          'https://raw.githubusercontent.com/vyfor/cord.nvim/refs/heads/master/.github/server-version.txt',
          '--fail',
          '--silent',
          '--show-error',
        },
      })
      :await()

  if res.code ~= 0 then
    error('Failed to fetch latest version; code: ' .. tostring(res.code), 0)
    if res.stderr and res.stderr ~= '' then logger.debug('curl stderr: ' .. res.stderr) end
    return nil
  end

  local version = res.stdout and res.stdout:gsub('^%s*(.-)%s*$', '%1') or nil
  if not version or version == '' then
    error('Failed to parse latest version', 0)
    return nil
  end

  return version
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
    local current, latest = M.local_version():await(), M.remote_version():await()

    if current and latest then
      if latest == current then
        logger.notify('You are on the latest server version ' .. latest, vim.log.levels.INFO)
      else
        logger.notify(
          'New version available: ' .. latest .. ' (current: ' .. current .. ')',
          vim.log.levels.INFO
        )
      end
    end
  end)
end)

M.version = async.wrap(function()
  async.run(function()
    local version = M.local_version():await()
    if version then logger.notify('Server version: ' .. version, vim.log.levels.INFO) end
  end)
end)

M.fetch = async.wrap(function()
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

  local fetch_executable = vim.schedule_wrap(function(tag)
    local base_url
    if tag then
      logger.info('Found new version: ' .. tag .. '. Downloading...')
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
        process
            .spawn({
              cmd = 'curl',
              args = {
                url,
                '--create-dirs',
                '--fail',
                '--location',
                '--silent',
                '--show-error',
                '-o',
                executable_path,
                '-H',
                'Accept: application/octet-stream',
              },
            })
            :and_then(function(res)
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
                require('cord.core.uv.fs').chmod(executable_path, '755'):await()
                server:initialize()
              end)
            end, function(err)
              server.is_updating = false
              logger.error(err)
            end)
      end)
    end

    if server.manager then server.manager:cleanup() end
    if not server.tx then return initialize() end
    if not server.client then return initialize() end
    if server.client:is_closing() then return initialize() end

    if server.client.on_close then server.client.on_close() end

    server.client.on_close = function()
      server.client.on_close = nil
      initialize()
    end

    server.tx:shutdown()
  end)

  async.run(function()
    logger.info 'Checking for updates...'
    local current, latest = M.local_version():await(), M.remote_version():await()

    if current and latest then
      if latest == current then
        server.is_updating = false
        logger.info('Already on latest server version ' .. latest)

        if not server.client or server.client:is_closing() then server:initialize() end
      else
        fetch_executable(latest)
      end
    else
      fetch_executable()
    end
  end)
end)

return M
