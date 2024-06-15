local log_level

local function init(level)
  log_level = vim.log.levels[string.upper(level)] or vim.log.levels.OFF

  if log_level == vim.log.levels.OFF then log_level = -1 end
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

local function log(status_code)
  if status_code == 2 then return error 'Provided client ID is not valid' end

  if status_code == 3 or status_code == 4 or status_code == 5 then
    return error 'Internal error occurred. Client will be disconnected'
  end

  if status_code == 6 then
    return info 'Current workspace is found in the blacklist. Presence will not be shown'
  end
end

return {
  init = init,
  info = info,
  warn = warn,
  error = error,
  debug = debug,
  log = log,
}
