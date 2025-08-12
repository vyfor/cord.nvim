local levels = vim.log.levels
local log_level = levels.ERROR

local queue = {}
local queue_start, queue_end = 1, 0
local flushing = false
local flush_scheduled = false

local function set_level(level) log_level = level end

local function enqueue(level, msg)
  queue_end = queue_end + 1
  queue[queue_end] = { level = level, msg = msg }
end

local function flush()
  flushing = true

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
        vim.notify('Error evaluating log message:\n' .. debug.traceback(), levels.WARN)
        return
      end
    else
      message = entry.msg
    end

    if message and (entry.level >= log_level) then
      vim.notify('[cord.nvim] ' .. message, entry.level)
    end
  end

  flushing = false
  flush_scheduled = false
end

local function flush_or_schedule()
  if flushing then return end

  if vim.in_fast_event and vim.in_fast_event() then
    if not flush_scheduled then
      flush_scheduled = true
      vim.schedule(flush)
    end
  else
    flush()
  end
end

local function log(level, msg)
  if not level or level < log_level then return end
  enqueue(level, msg)
  flush_or_schedule()
end

local function log_raw(level, msg)
  if not level then return end
  enqueue(level, msg)
  flush_or_schedule()
end

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
