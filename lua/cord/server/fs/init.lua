local M = {}

function M.get_plugin_root()
  local source = debug.getinfo(1, 'S').source:sub(2)
  return vim.fn.fnamemodify(source, ':h:h:h:h')
end

function M.get_data_path() return vim.fn.stdpath 'data' .. '/cord' end

---@param config? CordConfig
function M.get_executable_path(config)
  return (config and config.advanced.server.executable_path or M.get_data_path())
    .. '/bin/'
    .. M.get_executable_name()
end

function M.get_executable_name()
  return require('cord.plugin.constants').get_os().name == 'windows' and 'cord.exe' or 'cord'
end

return M
