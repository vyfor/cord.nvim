local log_level

local function init(level)
  log_level = level and vim.log.levels[string.upper(level)]
    or vim.log.levels.OFF

  return log_level
end

local function info(msg)
  if vim.log.levels.INFO >= log_level then
    vim.notify('[cord.nvim] ' .. msg, vim.log.levels.INFO)
  end
end

local function warn(msg)
  if vim.log.levels.WARN >= log_level then
    vim.notify('[cord.nvim] ' .. msg, vim.log.levels.WARN)
  end
end

local function error(msg)
  if vim.log.levels.ERROR >= log_level then
    vim.notify('[cord.nvim] ' .. msg, vim.log.levels.ERROR)
  end
end

local function debug(msg)
  if vim.log.levels.DEBUG >= log_level then
    vim.notify('[cord.nvim] ' .. msg, vim.log.levels.DEBUG)
  end
end

local function log(level, message)
  if level >= log_level then vim.notify('[cord.nvim] ' .. message, level) end
end

return {
  init = init,
  info = info,
  warn = warn,
  error = error,
  debug = debug,
  log = log,
}
