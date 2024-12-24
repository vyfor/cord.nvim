local M = {}

M.default_icon = 'plugin'
local mappings = {
  lazy = { M.default_icon, 'Lazy' },
  pckr = { M.default_icon, 'Pckr' },
  packer = { M.default_icon, 'Packer' },
}

M.get = function(filetype) return mappings[filetype] end

return M
