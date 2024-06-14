local function info(msg) vim.notify('[cord.nvim] ' .. msg, vim.log.levels.INFO) end

local function warn(msg) vim.notify('[cord.nvim] ' .. msg, vim.log.levels.WARN) end

local function error(msg)
  vim.notify('[cord.nvim] ' .. msg, vim.log.levels.ERROR)
end

local function debug(msg)
  vim.notify('[cord.nvim] ' .. msg, vim.log.levels.DEBUG)
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
  info = info,
  warn = warn,
  error = error,
  debug = debug,
  log = log,
}
