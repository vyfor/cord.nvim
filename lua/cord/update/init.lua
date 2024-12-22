local logger = require 'cord.util.logger'

local function build(callback)
  if not vim.fn.executable 'cargo' then
    logger.error 'cargo is not installed or not in PATH'
    return
  end

  local utils = require 'cord.util'
  local uv = vim.loop or vim.uv

  logger.info 'Building executable...'

  local pid = vim.g.cord_pid
  if pid then utils.kill_process(pid) end
  uv.spawn('cargo', {
    args = {
      'install',
      'cord-nvim',
      '--root',
      require('cord.util.file_manager').get_data_path(),
    },
    stdio = { nil, nil, nil },
  }, function(code, signal)
    if code ~= 0 then
      logger.error('Failed to build executable: ' .. code .. ' ' .. signal)
      return
    end

    logger.info 'Successfully built executable. Restarting...'
    require('cord'):cleanup()
    require('cord'):initialize()

    if callback then callback() end
  end)
end

local function fetch(callback)
  if not vim.fn.executable 'curl' then
    logger.error 'curl is not installed or not in PATH'
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
        return
      end

      if utils.os_name ~= 'windows' then
        uv.fs_chmod(executable_path, '755', function(err)
          if err then
            logger.error('Failed to set executable permissions: ' .. err)
            return
          end

          logger.info 'Successfully updated executable. Restarting...'
          require('cord'):cleanup()
          require('cord'):initialize()

          if callback then callback() end
        end)
      else
        logger.info 'Successfully updated executable. Restarting...'
        require('cord'):cleanup()
        require('cord'):initialize()

        if callback then callback() end
      end
    end)
  end

  uv.fs_stat(executable_path, function(err)
    if err then
      fetch_executable()
    else
      logger.info 'Checking for updates...'
      client.get(
        {
          'https://api.github.com/repos/vyfor/cord.nvim/releases/latest',
          '--fail',
        },
        vim.schedule_wrap(function(chunk, err)
          if err then
            logger.error('Failed to check for updates: ' .. err)
            return
          end

          local ok, data = pcall(vim.fn.json_decode, chunk)
          if not ok then
            logger.error('Failed to parse JSON response: ' .. data)
            return
          end

          local tag = data.tag_name
          if not tag then
            logger.error 'No tag found in GitHub response'
            return
          end

          if tag == require('cord.util.constants').VERSION then
            logger.info 'Already on latest version'
          else
            fetch_executable(tag)
          end
        end)
      )
    end
  end)
end

return {
  build = build,
  fetch = fetch,
}
