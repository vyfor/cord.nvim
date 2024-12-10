local M = {}

local mappings = {
  alpha = { 'default', 'Alpha' },
  dashboard = { 'default', 'Dashboard' },
  dashboardpreview = { 'default', 'Dashboard' },
  ministarter = { 'default', 'mini.starter' },
  startify = { 'default', 'Startify' },
}

M.get = function(filetype) return mappings[filetype] end

return M
