local M = {}

M.get = function(args, callback)
  local logger = require 'cord.util.logger'
  local stdout = vim.loop.new_pipe()
  local stderr = vim.loop.new_pipe()
  local chunks = {}
  local error_output = ''

  logger.debug('Running curl ' .. table.concat(args, ' '))

  local handle
  handle = vim.loop.spawn('curl', {
    args = args,
    stdio = { nil, stdout, stderr },
    hide = true,
  }, function(code, _)
    if code ~= 0 then
      logger.error('curl exited with code: ' .. code)
      stdout:close()
      stderr:close()
      handle:close()
      return callback(
        nil,
        'curl exited with code: ' .. code .. '\nError: ' .. error_output
      )
    else
      logger.debug('curl exited with code: ' .. code)
    end

    logger.debug 'Reading response...'

    stdout:read_start(function(err, chunk)
      if err then
        logger.error('curl read error: ' .. err)
        stdout:close()
        stderr:close()
        callback(nil, 'Read error: ' .. err)
      elseif chunk then
        logger.debug 'Read chunk'
        table.insert(chunks, chunk)
      else
        logger.debug 'Finished reading response (reached EOF)'
        stdout:close()
        stderr:close()
        callback(table.concat(chunks))
      end
    end)
    handle:close()
  end)

  if not handle then
    logger.error 'curl failed to spawn'
    stdout:close()
    stderr:close()
    callback(nil, 'Failed to spawn curl process')
    return
  end

  stderr:read_start(function(err, data)
    assert(not err, err)
    if data then error_output = error_output .. data end
  end)
  logger.debug('error stream: ' .. error_output)
end

M.execute = function(args, callback)
  local stderr = vim.loop.new_pipe(false)
  local error_output = ''

  local handle
  handle = vim.loop.spawn('curl', {
    args = args,
    stdio = { nil, nil, stderr },
    hide = true,
  }, function(code, _)
    stderr:read_stop()
    stderr:close()
    handle:close()

    if code ~= 0 then
      callback('curl exited with code: ' .. code .. '\nError: ' .. error_output)
    else
      callback()
    end
  end)

  if not handle then
    stderr:close()
    callback 'Failed to spawn curl process'
    return
  end

  stderr:read_start(function(err, data)
    assert(not err, err)
    if data then error_output = error_output .. data end
  end)
end

return M
