local M = {}

M.default_icon = 'git'
local mappings = {
  magit = { M.default_icon, 'Magit' },
  gitcommit = { M.default_icon, 'Git' },
  gitrebase = { M.default_icon, 'Git' },
  fugitive = { M.default_icon, 'Fugitive' },
  fugitiveblame = { M.default_icon, 'Fugitive' },
  lazygit = { M.default_icon, 'Lazygit' },
  ['git.nvim'] = { M.default_icon, 'Git.nvim' },
}

M.get = function(filetype)
  if filetype:match '^Neogit' then return { M.default_icon, 'Neogit' } end

  return mappings[filetype]
end

return M
