local M = {}

M.default_icon = 'folder'
local mappings = {
  netrw = { M.default_icon, 'Netrw' },
  TelescopePrompt = { 'telescope', 'Telescope' },
  dirvish = { M.default_icon, 'Dirvish' },
  fern = { M.default_icon, 'Fern' },
  oil = { M.default_icon, 'Oil' },
  oil_preview = { M.default_icon, 'Oil' },
  oil_progress = { M.default_icon, 'Oil' },
  NvimTree = { M.default_icon, 'nvim-tree' },
  minifiles = { M.default_icon, 'mini.files' },
  yazi = { M.default_icon, 'Yazi' },
  ['neo-tree'] = { M.default_icon, 'Neo-Tree' },
}

M.get = function(filetype) return mappings[filetype] end

return M
