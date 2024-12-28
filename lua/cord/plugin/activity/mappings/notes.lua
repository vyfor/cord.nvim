local M = {}

M.default_icon = 'notes'
local mappings = {
  norg = { 'neorg', 'Neorg' },
  org = { 'org', 'Orgmode' },
  ['org-roam-node-buffer'] = { 'org', 'Orgmode' },
  ['org-roam-select'] = { 'org', 'Orgmode' },
}

M.get = function(filetype) return mappings[filetype] end

return M
