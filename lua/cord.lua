local M = {}

---@param opts? CordConfig Configuration options for Cord
---@return nil
function M.setup(opts)
  local config = require('cord.plugin.config.util'):validate(opts or {})
  if config then require('cord.server'):initialize(config) end
end

return M
