local levels = vim.log.levels
local log_level = levels.ERROR

local function set_level(level) log_level = level end

local function log(level, msg)
  if level >= log_level then
    if vim.in_fast_event() then
      vim.schedule(function() vim.notify('[cord.nvim] ' .. msg, level) end)
    else
      vim.notify('[cord.nvim] ' .. msg, level)
    end
  end
end

local function log_raw(level, msg)
  if vim.in_fast_event() then
    vim.schedule(function() vim.notify('[cord.nvim] ' .. msg, level) end)
  else
    vim.notify('[cord.nvim] ' .. msg, level)
  end
end

local function logcb(level, cb)
  if level >= log_level then
    local msg = cb()
    if not msg then return end

    log_raw(level, msg)
  end
end

local function info(msg) log(levels.INFO, msg) end

local function warn(msg) log(levels.WARN, msg) end

local function error(msg) log(levels.ERROR, msg) end

local function debug(msg) log(levels.DEBUG, msg) end

local function trace(msg) log(levels.TRACE, msg) end

local function infocb(cb) logcb(levels.INFO, cb) end

local function warncb(cb) logcb(levels.WARN, cb) end

local function errorcb(cb) logcb(levels.ERROR, cb) end

local function debugcb(cb) logcb(levels.DEBUG, cb) end

local function tracecb(cb) logcb(levels.TRACE, cb) end

return {
  set_level = set_level,
  log = log,
  log_raw = log_raw,
  logcb = logcb,
  info = info,
  warn = warn,
  error = error,
  debug = debug,
  trace = trace,
  infocb = infocb,
  warncb = warncb,
  errorcb = errorcb,
  debugcb = debugcb,
  tracecb = tracecb,
}
