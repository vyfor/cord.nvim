local M = {}

M.default_icon = 'book'
local mappings = {
  help = { M.default_icon, 'Vim documentation' },
  help_ru = { M.default_icon, 'Vim documentation' },
  man = { M.default_icon, 'Man pages' },
}

M.get = function(filetype) return mappings[filetype] end

return M
