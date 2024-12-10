local M = {}

local mappings = {
  mason = { 'mason', 'Mason' },
  lspinfo = { 'lsp', 'LSP Info' },
}

M.get = function(filetype) return mappings[filetype] end

return M
