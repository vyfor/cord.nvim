local logger = require 'cord.util.logger'

local function build(callback)
  if vim.g.cord_is_updating then return end
  vim.g.cord_is_updating = true

  if not vim.fn.executable 'cargo' then
    logger.error 'cargo is not installed or not in PATH'
    vim.g.cord_is_updating = false
    return
  end

  local utils = require 'cord.util'
  local uv = vim.loop or vim.uv

  logger.info 'Building executable...'

  local pid = vim.g.cord_pid
  if pid then utils.kill_process(pid) end

  local stderr = uv.new_pipe()
  local error_output = ''

  local handle
  handle = uv.spawn('cargo', {
    args = {
      'install',
      'cord-nvim',
      '--force',
      '--root',
      require('cord.util.file_manager').get_data_path(),
    },
  }, function(code, _)
    if code ~= 0 then
      stderr:close()
      handle:close()
      logger.error(
        'Failed to build executable: cargo exited with code '
          .. code
          .. '\nError: '
          .. error_output
      )
      vim.g.cord_is_updating = false
      return
    end

    logger.info 'Successfully built executable. Restarting...'
    vim.g.cord_is_updating = false

    require('cord'):cleanup()
    require('cord'):initialize()

    if callback then callback() end
  end)

  if not handle then
    logger.error 'Failed to spawn cargo process'
    vim.g.cord_is_updating = false
    return
  end

  stderr:read_start(function(err, chunk)
    if err then
      stderr:close()
      handle:close()
      logger.error('Failed to read stderr: ' .. err)
      vim.g.cord_is_updating = false
    elseif chunk then
      error_output = error_output .. chunk
    end
  end)
end

local function get_version(executable, callback)
  local uv = vim.loop or vim.uv

  local handle = uv.new_pipe()
  uv.spawn(executable, {
    args = { '-v' },
    stdio = { nil, handle, nil },
  }, function(code, _)
    if code == 0 then
      handle:read_start(function(_, data)
        local version
        if data then version = data:gsub('^%s*(.-)%s*$', '%1') end
        handle:close()
        callback(version)
      end)
    else
      callback()
    end
  end)
end

local function fetch(callback)
  if vim.g.cord_is_updating then return end
  vim.g.cord_is_updating = true

  if not vim.fn.executable 'curl' then
    logger.error 'curl is not installed or not in PATH'
    vim.g.cord_is_updating = false
    return
  end

  local uv = vim.loop or vim.uv
  local client = require 'cord.http'
  local file_manager = require 'cord.util.file_manager'
  local utils = require 'cord.util'

  local executable_path = file_manager.get_data_path()
    .. utils.path_sep
    .. 'bin'
    .. utils.path_sep
    .. file_manager.get_executable_name()

  local fetch_executable = function(tag)
    local pid = vim.g.cord_pid
    if pid then utils.kill_process(pid) end

    local base_url
    if tag then
      logger.info('Found new version: ' .. tag .. '. Downloading...')
      base_url = 'https://github.com/vyfor/cord.nvim/releases/download/'
        .. tag
        .. '/'
    else
      logger.info 'Downloading latest version...'
      base_url = 'https://github.com/vyfor/cord.nvim/releases/latest/download/'
    end

    client.execute({
      base_url
        .. utils.os_arch
        .. '-'
        .. utils.os_name
        .. '-'
        .. (utils.os_name == 'windows' and 'cord.exe' or 'cord'),
      '--create-dirs',
      '--fail',
      '--location',
      '-o',
      executable_path,
      '-H',
      'Accept: application/octet-stream',
    }, function(err)
      if err then
        logger.error('Failed to download update: ' .. err)
        vim.g.cord_is_updating = false
        return
      end

      if utils.os_name ~= 'windows' then
        uv.fs_chmod(executable_path, '755', function(err)
          if err then
            logger.error('Failed to set executable permissions: ' .. err)
            vim.g.cord_is_updating = false
            return
          end

          logger.info 'Successfully updated executable. Restarting...'
          vim.g.cord_is_updating = false

          require('cord'):cleanup()
          require('cord'):initialize()

          if callback then callback() end
        end)
      else
        logger.info 'Successfully updated executable. Restarting...'
        vim.g.cord_is_updating = false

        require('cord'):cleanup()
        require('cord'):initialize()

        if callback then callback() end
      end
    end)
  end

  uv.fs_stat(executable_path, function(err)
    if err then
      logger.debug 'Version check failed, fetching latest...'
      fetch_executable()
    else
      logger.info 'Checking for updates...'

      get_version(executable_path, function(version)
        if not version then
          logger.debug 'Version check failed, fetching latest...'
          fetch_executable()
        else
          logger.debug('Found local version: ' .. version)
          client.get(
            {
              'https://api.github.com/repos/vyfor/cord.nvim/releases/latest',
              '--fail',
            },
            vim.schedule_wrap(function(chunk, err)
              if err then
                logger.error('Failed to check for updates: ' .. err)
                vim.g.cord_is_updating = false
                return
              end

              local ok, data = pcall(vim.fn.json_decode, chunk)
              if not ok then
                logger.error('Failed to parse JSON response: ' .. data)
                vim.g.cord_is_updating = false
                return
              end

              local tag = data.tag_name
              if not tag then
                logger.error 'No tag found in GitHub response'
                vim.g.cord_is_updating = false
                return
              end

              logger.debug(
                'Latest version: ' .. tag .. ', Current version: ' .. version
              )
              if tag == version then
                logger.info 'Already on latest version'
                vim.g.cord_is_updating = false
              else
                fetch_executable(tag)
              end
            end)
          )
        end
      end)
    end
  end)
end

return {
  build = build,
  fetch = fetch,
}
