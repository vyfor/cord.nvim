local M = {}

local mappings = {
  magit = { 'default', 'Magit' },
  gitcommit = { 'default', 'Git' },
  gitrebase = { 'default', 'Git' },
  fugitive = { 'default', 'Fugitive' },
  fugitiveblame = { 'default', 'Fugitive' },
  lazygit = { 'default', 'Lazygit' },
  ['git.nvim'] = { 'default', 'Git.nvim' },
}

M.get = function(filetype)
  if filetype:match '^Neogit' then return { 'default', 'Neogit' } end

  return mappings[filetype]
end

return M
