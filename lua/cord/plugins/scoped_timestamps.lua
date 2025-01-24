local user_config = require 'cord.plugin.config'

local M = {
  timestamps = {}, -- Effective timestamp for each key
  inactive_time = {}, -- Time spent inactive for each key
  last_active = {}, -- When key was last active
  current_key = nil,
  config = {
    scope = 'buffer',
    pause = true,
  },
}

local function get_key(opts)
  if M.config.scope == 'buffer' then
    if opts.type == 'language' then
      local key = vim.api.nvim_buf_get_name(0)
      if key == '' then return 'Cord.unknown' end
      return key
    else
      return opts.filetype
    end
  elseif M.config.scope == 'workspace' then
    return opts.workspace_dir
  elseif M.config.scope == 'idle' then
    if not opts.is_idle then return 'Cord.active' end
    return 'Cord.idle'
  end
end

local function handle_buffer_leave()
  if M.current_key and M.last_active[M.current_key] then
    local elapsed = os.time() - M.last_active[M.current_key]
    if elapsed > 0 then
      M.inactive_time[M.current_key] = (M.inactive_time[M.current_key] or 0) + elapsed
    end
    M.last_active[M.current_key] = nil
    M.timestamps[M.current_key] = nil
    M.current_key = nil
  end
end

M.setup = function(config)
  if config then M.config = vim.tbl_deep_extend('force', M.config, config) end

  if M.config.pause then
    local group = vim.api.nvim_create_augroup('CordScopedTimestampsPlugin', { clear = true })
    vim.api.nvim_create_autocmd('BufLeave', {
      group = group,
      callback = handle_buffer_leave,
    })
  end

  return {
    name = 'Scoped Timestamps',
    description = 'Tracks scoped timestamps for buffers and workspaces',

    variables = {
      get_scoped_timestamp = function(opts)
        local key = get_key(opts)
        if not key then return end

        if M.timestamps[key] then
          if not M.config.pause then return M.timestamps[key] end
          if M.current_key == key then return M.timestamps[key] end
        end

        if
          M.config.pause
          and M.current_key
          and M.current_key ~= key
          and M.last_active[M.current_key]
        then
          local elapsed = os.time() - M.last_active[M.current_key]
          if elapsed > 0 then
            M.inactive_time[M.current_key] = (M.inactive_time[M.current_key] or 0) + elapsed
          end
          M.last_active[M.current_key] = nil
          M.timestamps[M.current_key] = nil
        end

        local current_time = os.time()
        if M.config.pause then
          M.current_key = key
          M.last_active[key] = current_time
          M.timestamps[key] = current_time - (M.inactive_time[key] or 0)
        else
          M.timestamps[key] = current_time
        end

        return M.timestamps[key]
      end,
    },

    hooks = user_config.timestamp.enabled and {
      post_activity = {
        function(opts, activity)
          local timestamp = opts.get_scoped_timestamp(opts)
          if timestamp then activity.timestamps.start = timestamp end
        end,
        priority = 0,
      },
      idle_enter = user_config.timestamp.reset_on_idle and {
        function()
          M.timestamps['Cord.idle'] = nil
          M.timestamps['Cord.active'] = nil
        end,
        priority = 0,
      } or nil,
      idle_leave = user_config.timestamp.reset_on_idle and {
        function()
          M.timestamps['Cord.idle'] = nil
          M.timestamps['Cord.active'] = nil
        end,
        priority = 0,
      } or nil,
    } or nil,
  }
end

return M
