local levels = vim.log.levels
local fs = require 'cord.core.uv.fs'
local Async = require 'cord.core.async'

local queue = {}
local queue_start, queue_end = 1, 0
local flushing = false
local fd

local function log_notify(msg, level)
  if vim.in_fast_event and vim.in_fast_event() then
    vim.schedule(function() vim.notify(msg, level) end)
  else
    vim.notify(msg, level)
  end
end

local logfile_path = os.getenv 'CORD_LOG_FILE'
if logfile_path and logfile_path ~= '' then
  logfile_path = vim.fn.fnamemodify(logfile_path, ':p')
else
  logfile_path = nil
  log_notify('[cord.nvim] CORD_LOG_FILE is not a valid path', levels.ERROR)
end

local level_names = {
  [levels.TRACE] = 'TRACE',
  [levels.DEBUG] = 'DEBUG',
  [levels.INFO] = 'INFO',
  [levels.WARN] = 'WARN',
  [levels.ERROR] = 'ERROR',
}

local function enqueue(level, msg)
  if not fd then
    if not logfile_path or logfile_path == '' then
      log_notify('[cord.nvim] CORD_LOG_FILE is not set', levels.ERROR)
      return
    end

    local ok, err = fs.mkdirp(logfile_path:match '^(.*)[/\\]'):get()
    if not ok then
      log_notify('[cord.nvim] Failed to create log file directory: ' .. tostring(err), levels.ERROR)
      return
    end

    local ok2, err2 = fs.openfile(logfile_path, 'w'):get()
    if err2 then
      log_notify('[cord.nvim] Failed to open log file: ' .. tostring(err2), levels.ERROR)
      return
    end

    fs.closefile(ok2)
    local ok3, err3 = fs.openfile(logfile_path, 'a'):get()
    if err3 then
      log_notify('[cord.nvim] Failed to open log file: ' .. tostring(err3), levels.ERROR)
      return
    end

    fd = ok3
  end

  queue_end = queue_end + 1
  queue[queue_end] = { level = level, msg = msg }
end

local function format_message(entry, message)
  local ts = os.date '%Y-%m-%d %H:%M:%S'
  local level_name = level_names[entry.level] or tostring(entry.level)
  return string.format('[%s] [cord.nvim] [%s] %s', ts, level_name, tostring(message))
end

local function flush()
  if not logfile_path or logfile_path == '' then
    flushing = false
    return
  end
  if not fd then return end
  if flushing then return end

  flushing = true

  local lines = {}
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

    if message ~= nil then table.insert(lines, format_message(entry, message)) end
  end

  local data = table.concat(lines, '\n')
  if #data > 0 then data = data .. '\n' end

  local ok, err = fs.write(fd, data):await()
  if not ok then
    log_notify('[cord.nvim] Failed to write log file: ' .. tostring(err), levels.ERROR)
  end

  flushing = false
end

local function log(level, msg)
  if not level then return end
  Async.run(function()
    enqueue(level, msg)
    flush()
  end)
end

local function log_raw(level, msg)
  if not level then return end
  Async.run(function()
    enqueue(level, msg)
    flush()
  end)
end

-- no-op
local function set_level(_level) end

local function error(msg) log(levels.ERROR, msg) end
local function warn(msg) log(levels.WARN, msg) end
local function info(msg) log(levels.INFO, msg) end
local function debug(msg) log(levels.DEBUG, msg) end
local function trace(msg) log(levels.TRACE, msg) end

return {
  set_level = set_level,
  log = log,
  log_raw = log_raw,
  error = error,
  warn = warn,
  info = info,
  debug = debug,
  trace = trace,
}
