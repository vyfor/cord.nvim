local levels = vim.log.levels
local fs = require 'cord.core.uv.fs'
local Async = require 'cord.core.async'
local msg_logger = require 'cord.api.log.notify'

local log_level = levels.TRACE
local queue = {}
local queue_start, queue_end = 1, 0
local flushing = false
local pending = false
local fd

local function set_level(level) log_level = level end

local function notify(msg, level) msg_logger.log_raw(msg, level) end

local logfile_path = os.getenv 'CORD_LOG_FILE'
if logfile_path and logfile_path ~= '' then
  logfile_path = vim.fn.fnamemodify(logfile_path, ':p')
else
  logfile_path = nil
  notify('CORD_LOG_FILE is not a valid path', levels.ERROR)
end

local level_names = {
  [levels.TRACE] = 'TRACE',
  [levels.DEBUG] = 'DEBUG',
  [levels.INFO] = 'INFO',
  [levels.WARN] = 'WARN',
  [levels.ERROR] = 'ERROR',
}

local function enqueue(msg, level, raw)
  if not fd then
    if not logfile_path or logfile_path == '' then
      notify('CORD_LOG_FILE is not set', levels.ERROR)
      return
    end

    local ok, err = fs.mkdirp(logfile_path:match '^(.*)[/\\]'):await()
    if not ok then
      notify('Failed to create log file directory: ' .. tostring(err), levels.ERROR)
      return
    end

    local ok2, err2 = fs.openfile(logfile_path, 'a'):await()
    if err2 then
      notify('Failed to open log file: ' .. tostring(err2), levels.ERROR)
      return
    end

    fd = ok2
  end

  queue_end = queue_end + 1
  queue[queue_end] = { level = level, msg = msg, raw = raw }
end

local function format_message(entry, message)
  local ts = os.date '%Y-%m-%d %H:%M:%S'
  local level_name = level_names[entry.level] or tostring(entry.level)
  return string.format('[%s] [%s] %s', ts, level_name, tostring(message))
end

local function drain(lines)
  while queue_start <= queue_end do
    local entry = queue[queue_start]
    queue[queue_start] = nil
    queue_start = queue_start + 1

    local message
    if type(entry.msg) == 'function' then
      local ok, res = pcall(entry.msg)
      if ok then
        message = res
      else
        message = 'Error evaluating log message: ' .. debug.traceback()
      end
    else
      message = entry.msg
    end

    if message ~= nil then
      if entry.raw then
        table.insert(lines, message)
      else
        table.insert(lines, format_message(entry, message))
      end
    end
  end
end

local function flush()
  if not logfile_path or logfile_path == '' then
    flushing = false
    pending = false
    return
  end
  if not fd then return end
  if flushing then
    pending = true
    return
  end

  flushing = true

  local lines = {}
  drain(lines)

  local data = table.concat(lines, '\n')
  if #data > 0 then data = data .. '\n' end

  local ok, err = fs.write(fd, data):unwrap()
  if not ok then notify('[cord.nvim] Failed to write log file: ' .. tostring(err), levels.ERROR) end

  flushing = false

  if pending then
    pending = false
    flush()
  end
end

local function flush_on_exit()
  if not logfile_path or logfile_path == '' then return end

  flushing = false
  pending = false

  local lines = {}
  drain(lines)

  local data = table.concat(lines, '\n')
  if #data > 0 then data = data .. '\n' end
  if #data == 0 then return end

  local f, err = io.open(logfile_path, 'a')
  if not f then
    notify('[cord.nvim] Failed to flush log file on exit: ' .. tostring(err), levels.ERROR)
    return
  end

  local ok, write_err = pcall(function()
    f:write(data)
    f:flush()
  end)
  f:close()
  if not ok then
    notify('[cord.nvim] Failed to flush log file on exit: ' .. tostring(write_err), levels.ERROR)
  end
end

vim.api.nvim_create_autocmd('VimLeavePre', {
  group = vim.api.nvim_create_augroup('CordFlush', { clear = true }),
  callback = flush_on_exit,
})

local function log(msg, level)
  if not level or level < log_level then return end
  Async.run(function()
    enqueue(msg, level)
    flush()
  end)
end

local function log_raw(msg, level)
  if not level then return end
  Async.run(function()
    enqueue(msg, level)
    flush()
  end)
end

local function log_server(logs)
  Async.run(function()
    local msgs = {}
    local ts = os.date '%Y-%m-%d %H:%M:%S'

    for _, item in ipairs(logs) do
      if item.level and item.level >= log_level then
        local level_name = level_names[item.level] or tostring(item.level)
        table.insert(msgs, string.format('[%s] [%s] [SERVER] %s', ts, level_name, item.message))
      end
    end

    if #msgs > 0 then
      enqueue(table.concat(msgs, '\n'), nil, true)
      flush()
    end
  end)
end

local function error(msg) log(msg, levels.ERROR) end
local function warn(msg) log(msg, levels.WARN) end
local function info(msg) log(msg, levels.INFO) end
local function debug(msg) log(msg, levels.DEBUG) end
local function trace(msg) log(msg, levels.TRACE) end

return {
  set_level = set_level,
  notify = notify,
  log = log,
  log_server = log_server,
  log_raw = log_raw,
  error = error,
  warn = warn,
  info = info,
  debug = debug,
  trace = trace,
}
