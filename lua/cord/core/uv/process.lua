local Future = require 'cord.core.async.future'
local uv = vim.loop or vim.uv

local M = {}

function M.spawn(options)
  return Future.new(function(resolve, reject)
    local stdout_data = {}
    local stderr_data = {}
    local stdout = uv.new_pipe()
    local stderr = uv.new_pipe()

    local handle, pid
    handle, pid = uv.spawn(options.cmd, {
      args = options.args,
      cwd = options.cwd,
      env = options.env,
      stdio = { nil, stdout, stderr },
      hide = options.hide,
      detached = options.detached,
    }, function(code, signal)
      stdout:read_stop()
      stderr:read_stop()
      stdout:close()
      stderr:close()
      handle:close()

      resolve {
        code = code,
        signal = signal,
        stdout = table.concat(stdout_data),
        stderr = table.concat(stderr_data),
        pid = pid,
      }
    end)

    if not handle then reject('Failed to spawn process: ' .. pid) end

    stdout:read_start(function(err, chunk)
      if err then reject(err) end
      if chunk then table.insert(stdout_data, chunk) end
    end)

    stderr:read_start(function(err, chunk)
      if err then reject(err) end
      if chunk then table.insert(stderr_data, chunk) end
    end)
  end)
end

function M.spawn_daemon(options)
  local stdout = uv.new_pipe()
  local stderr = uv.new_pipe()
  local buffer = { stdout = '', stderr = '' }

  local handle, pid
  handle, pid = uv.spawn(options.cmd, {
    args = options.args,
    cwd = options.cwd,
    env = options.env,
    stdio = { nil, stdout, stderr },
    hide = true,
    detached = true,
  }, function(code, signal)
    handle:close()

    if options.on_exit then options.on_exit(code, signal) end
  end)

  if not handle then
    if stdout then stdout:close() end
    if stderr then stderr:close() end
    error('Failed to spawn process: ' .. pid)
  end

  stdout:read_start(function(err, chunk)
    if err then
      stdout:read_stop()
      stdout:close()
      if options.on_error then options.on_error(err) end
      return
    end
    if chunk then
      buffer.stdout = buffer.stdout .. chunk

      while true do
        local line_end = buffer.stdout:find '\n'
        if not line_end then break end

        local line = buffer.stdout:sub(1, line_end - 1)
        if options.on_stdout then options.on_stdout(line) end

        buffer.stdout = buffer.stdout:sub(line_end + 1)
      end
    else
      stdout:read_stop()
      stdout:close()
    end
  end)

  stderr:read_start(function(err, chunk)
    if err then
      stderr:read_stop()
      stderr:close()
      if options.on_error then options.on_error(err) end
      return
    end
    if chunk then
      buffer.stderr = buffer.stderr .. chunk

      local line_end = buffer.stderr:find '\n'
      if line_end then
        local line = buffer.stderr:sub(1, line_end - 1)
        stderr:read_stop()
        stderr:close()
        if options.on_stderr then options.on_stderr(line) end
      end
    else
      stderr:read_stop()
      stderr:close()
    end
  end)

  if options.detached then handle:unref() end

  return handle
end

return M
