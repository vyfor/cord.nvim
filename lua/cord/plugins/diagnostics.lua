local M = {
  diagnostic_count = 0,
  config = {
    scope = 0,
    severity = { min = vim.diagnostic.severity.WARN },
    override = true,
  },
}

local function create_problems_suffix(problems_count)
  local text = ' - ' .. problems_count
  if problems_count == 1 then
    text = text .. ' problem'
  else
    text = text .. ' problems'
  end
  return text
end

M.setup = function(config)
  if config then
    config = vim.tbl_deep_extend('force', M.config, config)

    local err = M.validate(config)
    if err then
      error(err, 0)
    else
      M.config = config
    end
  end

  return {
    name = 'Diagnostics',
    description = 'Displays diagnostic information',

    variables = {
      diagnostic = function() return M.diagnostics end,
      diagnostics = function() return M.diagnostic_count end,
    },

    config = M.config.override and {
      text = {
        workspace = M.config.scope == nil and function(opts)
          local text = 'In ' .. opts.workspace
          if M.diagnostic_count > 0 then
            text = text .. create_problems_suffix(M.diagnostic_count)
          end
          return text
        end or nil,
        viewing = M.config.scope == 0 and function(opts)
          local text = 'Viewing ' .. opts.filename
          if M.diagnostic_count > 0 then
            text = text .. create_problems_suffix(M.diagnostic_count)
          end
          return text
        end or nil,
        editing = M.config.scope == 0 and function(opts)
          local text = 'Editing ' .. opts.filename
          if M.diagnostic_count > 0 then
            text = text .. create_problems_suffix(M.diagnostic_count)
          end
          return text
        end or nil,
      },
    } or nil,
  }
end

M.validate = function(config)
  if config.scope then
    if config.scope == 'buffer' then
      config.scope = 0
    elseif config.scope == 'workspace' then
      config.scope = nil
    else
      return 'Invalid scope value, must be \'buffer\' or \'workspace\''
    end
  end

  if config.severity then
    local ty = type(config.severity)
    if ty ~= 'string' and ty ~= 'number' and ty ~= 'table' then
      return 'Invalid severity value, must be a string, number or table'
    end
  end

  if config.override then
    if type(config.override) ~= 'boolean' then
      return 'Invalid override value, must be a boolean'
    end
  end
end

M.get_diagnostics = function()
  return vim.diagnostic.get(M.config.scope, { severity = M.config.severity })
end

vim.api.nvim_create_autocmd('DiagnosticChanged', {
  callback = function()
    M.diagnostics = M.get_diagnostics()
    M.diagnostic_count = #M.diagnostics
  end,
  group = vim.api.nvim_create_augroup('CordDiagnosticsPlugin', { clear = true }),
})

return M
