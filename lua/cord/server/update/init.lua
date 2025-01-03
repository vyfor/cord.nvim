local async = require 'cord.core.async'
local logger = require 'cord.plugin.log'

local M = {}

M.build = async.wrap(function()
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
              if res.stderr then
                logger.error('cargo\'s stderr: ' .. res.stderr)
              end
              return
            end
            logger.info 'Successfully built executable. Restarting...'

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

M.fetch = async.wrap(function()
  local server = require 'cord.server'
  if server.is_updating then return end
  server.is_updating = true

  if not vim.fn.executable 'curl' then
    error('curl is not installed or not in PATH', 0)
    return
  end

  local executable_path = require('cord.server.fs').get_executable_path()
  local process = require 'cord.core.uv.process'

  local fetch_executable = vim.schedule_wrap(function(tag)
    local base_url
    if tag then
      logger.info('Found new version: ' .. tag .. '. Downloading...')
      base_url = 'https://github.com/vyfor/cord.nvim/releases/download/v'
        .. tag
        .. '/'
    else
      logger.info 'Downloading latest version...'
      base_url = 'https://github.com/vyfor/cord.nvim/releases/latest/download/'
    end

    local os_info = require('cord.plugin.constants').get_os()
    local url = base_url
      .. os_info.arch
      .. '-'
      .. os_info.name
      .. '-'
      .. (os_info.name == 'windows' and 'cord.exe' or 'cord')

    local cord = require 'cord.server'
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
              '-o',
              executable_path,
              '-H',
              'Accept: application/octet-stream',
            },
          })
          :and_then(function(res)
            if res.code ~= 0 then
              server.is_updating = false
              logger.error(
                'Failed to download executable; code: '
                  .. res.code
                  .. ', path: '
                  .. url
              )
              if res.stderr then
                logger.error('curl\'s stderr: ' .. res.stderr)
              end
              return
            end
            logger.info 'Successfully updated executable. Restarting...'

            async.run(function()
              server.is_updating = false
              require('cord.core.uv.fs').chmod(executable_path, '755'):await()
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

  async.run(function()
    logger.info 'Checking for updates...'
    process
      .spawn({
        cmd = executable_path,
        args = { '-v' },
      })
      :and_then(function(res)
        if res.code ~= 0 then
          fetch_executable()
          return
        end

        local version = res.stdout:gsub('^%s*(.-)%s*$', '%1')
        if not version then
          fetch_executable()
          return
        end

        logger.debug('Local version: ' .. version)

        async.run(function()
          process
            .spawn({
              cmd = 'curl',
              args = {
                'https://raw.githubusercontent.com/vyfor/cord.nvim/refs/heads/client-server/.github/server-version.txt',
                '--fail',
              },
            })
            :and_then(vim.schedule_wrap(function(res)
              async.run(function()
                if res.code == 0 then
                  local latest = res.stdout:gsub('^%s*(.-)%s*$', '%1')
                  if not latest then
                    error(
                      'Failed to parse latest release; code: ' .. res.code,
                      0
                    )
                    return
                  end

                  if latest == version then
                    server.is_updating = false
                    logger.info('Already on latest server version ' .. latest)
                    return
                  end

                  fetch_executable(latest)
                else
                  error('Failed to fetch latest release: ' .. res.stdout, 0)
                end
              end)
            end))
            :catch(function(err)
              server.is_updating = false
              logger.error('Failed to fetch latest release: ' .. err)
            end)
        end)
      end, function() fetch_executable() end)
  end)
end)

return M
