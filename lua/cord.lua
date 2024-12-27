local M = {}

---@param opts? CordConfig Configuration options for Cord
---@return nil
function M.setup(opts)
  if require('cord.plugin.config.util'):validate(opts or {}) then
    require('cord.core.async').run(
      function() return require('cord.server').initialize():await() end
    )
  end
end

return M
