local M

M = {
  ---@param opts? CordConfig Configuration options for Cord
  setup = function(opts)
    if opts then M.user_config = opts end
  end,
}

return M
