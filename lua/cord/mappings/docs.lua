local M = {}

local mappings = {
  help = { 'default', 'Vim documentation' },
  help_ru = { 'default', 'Vim documentation' },
  man = { 'default', 'Man pages' },
}

M.get = function(filetype) return mappings[filetype] end

return M
