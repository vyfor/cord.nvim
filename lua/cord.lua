local M

M = {
  ---@param opts? CordConfig Configuration options for Cord
  setup = function(opts)
    if opts then M.user_config = opts end

    if vim.g.cord_defer_startup == true then
      local config = require('cord.plugin.config.util').validate(opts)
      if not config then return end

      if config.enabled then
        vim.cmd [[
            augroup Cord
                autocmd!
                autocmd VimLeavePre * lua require 'cord.server':cleanup()
            augroup END
        ]]

        require('cord.server'):initialize()
      end
    end
  end,
}

return M
