local M = {}

M.default_icon = 'dashboard'
local mappings = {
  alpha = { M.default_icon, 'Alpha' },
  dashboard = { M.default_icon, 'Dashboard' },
  dashboardpreview = { M.default_icon, 'Dashboard' },
  ministarter = { M.default_icon, 'mini.starter' },
  startify = { M.default_icon, 'Startify' },
}

M.get = function(filetype) return mappings[filetype] end

return M
