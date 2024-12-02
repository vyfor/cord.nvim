local M = {}

local mappings = {
  netrw = { 'default', 'Netrw' },
  TelescopePrompt = { 'telescope', 'Telescope' },
  dirvish = { 'default', 'Dirvish' },
  fern = { 'default', 'Fern' },
  oil = { 'default', 'Oil' },
  NvimTree = { 'default', 'nvim-tree' },
  minifiles = { 'default', 'mini.files' },
  yazi = { 'default', 'Yazi' },
  ['neo-tree'] = { 'default', 'Neo-Tree' },
}

M.get = function(filetype) return mappings[filetype] end

return M
