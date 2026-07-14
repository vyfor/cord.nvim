local M = {
  in_tmux = false,
  pane = nil,
  session = nil,
  window = nil,
  attached_count = nil,

  config = {
    interval = 20000,
    focus_events = true,
    on_attach = 'show',
    on_detach = 'hide',
  },
}

M.setup = function(config)
  M.config = vim.tbl_deep_extend('force', M.config, config or {})

  local uv = vim.uv or vim.loop
  local async = require 'cord.core.async'
  local process = require 'cord.core.uv.process'
  local timer
  local update_state

  return {
    name = 'tmux',
    description = 'Provides tmux-related hooks and status tracking',

    variables = {
      in_tmux = function() return M.in_tmux end,
      tmux_pane = function() return M.pane end,
      tmux_session = function() return M.session end,
      tmux_window = function() return M.window end,
      attached_count = function() return M.attached_count end,
    },

    hooks = {
      ready = async.wrap(function(manager)
        local tmux_pane = vim.env.TMUX_PANE
        if not tmux_pane or tmux_pane == '' then return end

        M.in_tmux = true
        M.pane = tmux_pane

        local was_detached = false

        update_state = function()
          async.run(function()
            local result, err = process
              .spawn({
                cmd = 'tmux',
                args = {
                  'display-message',
                  '-t',
                  tmux_pane,
                  '-p',
                  '#{session_attached}:#{session_name}:#{window_name}',
                },
              })
              :await()

            if err or result.code ~= 0 then return end

            local parts = vim.split(vim.trim(result.stdout or ''), ':')
            if #parts < 3 then return end

            local attached_count = tonumber(parts[1]) or 0
            M.attached_count = attached_count
            M.session = parts[2]
            M.window = parts[3]

            local is_detached = attached_count == 0

            if is_detached then
              if not was_detached then
                was_detached = true
                if M.config.on_detach == 'hide' then
                  manager:suppress()
                elseif M.config.on_detach == 'idle' then
                  manager:idle()
                elseif type(M.config.on_detach) == 'function' then
                  M.config.on_detach()
                end
              end
            else
              if was_detached then
                was_detached = false
                if M.config.on_attach == 'show' then
                  if M.config.on_detach == 'hide' then
                    manager:resume()
                  elseif M.config.on_detach == 'idle' then
                    manager:unidle()
                  end
                elseif type(M.config.on_attach) == 'function' then
                  M.config.on_attach()
                end
              end
            end
          end)
        end

        if type(M.config.interval) == 'number' and M.config.interval > 0 then
          timer = uv.new_timer()
          if timer then timer:start(0, M.config.interval, vim.schedule_wrap(update_state)) end
        else
          vim.schedule(update_state)
        end
      end),

      focus_gained = function()
        if M.config.focus_events and update_state then update_state() end
      end,

      focus_lost = function()
        if M.config.focus_events and update_state then update_state() end
      end,

      shutdown = function()
        if timer then
          timer:stop()
          timer:close()
          timer = nil
        end
      end,
    },
  }
end

return M
