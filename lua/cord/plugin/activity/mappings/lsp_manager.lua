local M = {}

M.default_icon = 'lsp'
local mappings = {
  mason = { M.default_icon, 'Mason' },
  lspinfo = { M.default_icon, 'LSP Info' },
}

M.get = function(filetype) return mappings[filetype] end

return M
