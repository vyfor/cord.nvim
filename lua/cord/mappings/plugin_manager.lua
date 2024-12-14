local M = {}

local mappings = {
  lazy = { 'default', 'Lazy' },
  pckr = { 'default', 'Pckr' },
  packer = { 'default', 'Packer' },
}

M.get = function(filetype) return mappings[filetype] end

return M
