local M = {
  time = (function()
    local date = os.date '*t'
    date.hour, date.min, date.sec = 0, 0, 0
    ---@diagnostic disable-next-line: param-type-mismatch
    return os.time(date)
  end)(),
  config = {
    affect_idle = true,
  },
}

M.setup = function(config)
  if config then M.config = vim.tbl_deep_extend('force', M.config, config) end

  return {
    name = 'Local Time',
    description = 'Displays the current local time',

    variables = {
      local_timestamp = function() return M.time end,
    },

    hooks = {
      post_activity = {
        fn = function(opts, activity)
          if not M.config.affect_idle and opts.is_idle then return end
          activity.timestamps.start = M.time
        end,
        priority = 0, -- run last
      },
    },
  }
end

return M
