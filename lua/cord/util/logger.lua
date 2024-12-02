local levels = vim.log.levels
local log_level

local function set_level(level) log_level = level end

local function log(level, msg)
  if levels[level] >= log_level then
    vim.notify('[cord.nvim] ' .. msg, level)
  end
end

local function log_raw(level, msg) vim.notify('[cord.nvim] ' .. msg, level) end

local function info(msg)
  if levels.INFO >= log_level then
    vim.notify('[cord.nvim] ' .. msg, levels.INFO)
  end
end

local function warn(msg)
  if levels.WARN >= log_level then
    vim.notify('[cord.nvim] ' .. msg, levels.WARN)
  end
end

local function error(msg)
  if levels.ERROR >= log_level then
    vim.notify('[cord.nvim] ' .. msg, levels.ERROR)
  end
end

local function debug(msg)
  if levels.DEBUG >= log_level then
    vim.notify('[cord.nvim] ' .. msg, levels.DEBUG)
  end
end

return {
  set_level = set_level,
  log = log,
  log_raw = log_raw,
  info = info,
  warn = warn,
  error = error,
  debug = debug,
}
