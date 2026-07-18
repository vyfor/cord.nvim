local M = {
  in_zellij = false,
  session = nil,
  attached_count = nil,

  config = {
    interval = 20000,
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
    name = 'zellij',
    description = 'Provides zellij-related hooks and status tracking',

    variables = {
      in_zellij = function() return M.in_zellij end,
      zellij_session = function() return M.session end,
      attached_count = function() return M.attached_count end,
    },

    hooks = {
      ready = async.wrap(function(manager)
        local zellij = vim.env.ZELLIJ
        if not zellij or zellij == '' then return end
        if vim.fn.executable 'zellij' ~= 1 then return end

        M.in_zellij = true
        M.session = vim.env.ZELLIJ_SESSION_NAME

        local was_detached = false

        update_state = function()
          async.run(function()
            local result, err = process
              .spawn({
                cmd = 'zellij',
                args = { 'action', 'list-clients' },
              })
              :await()

            if err or result.code ~= 0 then return end

            local stdout = vim.trim(result.stdout or '')
            if stdout == '' then return end

            local attached_count = 0
            for _, line in ipairs(vim.split(stdout, '\n')) do
              if line ~= '' and not vim.startswith(line, 'CLIENT_ID') then
                attached_count = attached_count + 1
              end
            end
            M.attached_count = attached_count

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
