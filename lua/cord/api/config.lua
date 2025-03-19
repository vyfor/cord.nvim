---@diagnostic disable-next-line: deprecated
local unpack = unpack or table.unpack

local M = {}

local validation_rules = {
  ['enabled'] = { 'boolean' },
  ['log_level'] = { 'string', 'number' },

  ['editor'] = { 'table' },
  ['editor.client'] = { 'string' },
  ['editor.tooltip'] = { 'string' },
  ['editor.icon'] = { 'string' },

  ['display'] = { 'table' },
  ['display.theme'] = { 'string' },
  ['display.swap_fields'] = { 'boolean' },
  ['display.swap_icons'] = { 'boolean' },
  ['display.flavor'] = { 'string' },  -- Added the missing entry

  ['timestamp'] = { 'table' },
  ['timestamp.enabled'] = { 'boolean' },
  ['timestamp.reset_on_idle'] = { 'boolean' },
  ['timestamp.reset_on_change'] = { 'boolean' },

  ['idle'] = { 'table' },
  ['idle.enabled'] = { 'boolean' },
  ['idle.timeout'] = { 'number' },
  ['idle.show_status'] = { 'boolean' },
  ['idle.ignore_focus'] = { 'boolean' },
  ['idle.unidle_on_focus'] = { 'boolean' },
  ['idle.smart_idle'] = { 'boolean' },
  ['idle.details'] = { 'string', 'function' },
  ['idle.state'] = { 'string', 'function' },
  ['idle.tooltip'] = { 'string', 'function' },
  ['idle.icon'] = { 'string', 'function' },

  ['text'] = { 'table' },
  ['text.workspace'] = { 'string', 'boolean', 'function' },
  ['text.viewing'] = { 'string', 'boolean', 'function' },
  ['text.editing'] = { 'string', 'boolean', 'function' },
  ['text.file_browser'] = { 'string', 'boolean', 'function' },
  ['text.plugin_manager'] = { 'string', 'boolean', 'function' },
  ['text.lsp'] = { 'string', 'boolean', 'function' },
  ['text.docs'] = { 'string', 'boolean', 'function' },
  ['text.vcs'] = { 'string', 'boolean', 'function' },
  ['text.notes'] = { 'string', 'boolean', 'function' },
  ['text.debug'] = { 'string', 'boolean', 'function' },
  ['text.test'] = { 'string', 'boolean', 'function' },
  ['text.games'] = { 'string', 'boolean', 'function' },
  ['text.diagnostics'] = { 'string', 'boolean', 'function' },
  ['text.terminal'] = { 'string', 'boolean', 'function' },
  ['text.dashboard'] = { 'string', 'boolean', 'function' },

  ['buttons'] = { 'table' },
  ['buttons.*.label'] = { 'string', 'function' },
  ['buttons.*.url'] = { 'string', 'function' },
  ['assets'] = { 'table' },
  ['variables'] = { 'boolean', 'table' },
  ['plugins'] = { 'table' },

  ['hooks'] = { 'table' },
  ['hooks.ready'] = { 'function', 'table' },
  ['hooks.shutdown'] = { 'function', 'table' },
  ['hooks.pre_activity'] = { 'function', 'table' },
  ['hooks.post_activity'] = { 'function', 'table' },
  ['hooks.idle_enter'] = { 'function', 'table' },
  ['hooks.idle_leave'] = { 'function', 'table' },
  ['hooks.workspace_change'] = { 'function', 'table' },

  ['advanced'] = { 'table' },
  ['advanced.plugin'] = { 'table' },
  ['advanced.plugin.autocmds'] = { 'boolean' },
  ['advanced.plugin.cursor_update'] = { 'string' },
  ['advanced.plugin.match_in_mappings'] = { 'boolean' },
  ['advanced.server'] = { 'table' },
  ['advanced.server.update'] = { 'string' },
  ['advanced.server.pipe_path'] = { 'string' },
  ['advanced.server.executable_path'] = { 'string' },
  ['advanced.server.timeout'] = { 'number' },
  ['advanced.discord'] = { 'table' },
  ['advanced.discord.reconnect'] = { 'table' },
  ['advanced.discord.reconnect.enabled'] = { 'boolean' },
  ['advanced.discord.reconnect.interval'] = { 'number' },
  ['advanced.discord.reconnect.initial'] = { 'boolean', 'table' },
}

local array_paths = {
  ['buttons'] = true,
  ['plugins'] = true,
}

local skip_subtrees = {
  ['plugins'] = true,
}

local dict_paths = {
  ['assets'] = { 'string', 'table' },
}

local function validate_type(value, allowed_types)
  for _, t in ipairs(allowed_types) do
    local ty = type(value)
    if t == 'table' and ty == 'table' then
      return true
    elseif t == 'string' and ty == 'string' then
      return true
    elseif t == 'number' and ty == 'number' then
      return true
    elseif t == 'boolean' and ty == 'boolean' then
      return true
    elseif t == 'function' and ty == 'function' then
      return true
    end
  end
  return false
end

local function is_valid_path(path)
  if validation_rules[path] then return true end

  local parts = vim.split(path, '.', { plain = true })
  if #parts >= 2 then
    local parent = parts[1]
    if dict_paths[parent] then return true end

    local wildcard_path = table.concat(
      vim.tbl_flatten {
        { parts[1] },
        { '*' },
        { unpack(parts, 3) },
      },
      '.'
    )
    return validation_rules[wildcard_path] ~= nil
  end
  return false
end

local function get_nested_value(config, path)
  local parts = vim.split(path, '.', { plain = true })
  local current = config
  for _, part in ipairs(parts) do
    if type(current) ~= 'table' then return nil end
    current = current[part]
  end
  return current
end

M.validate = function(user_config)
  if not user_config then return { is_valid = true } end

  local errors = {}
  local warnings = {}

  local function check_unknown_entries(config, prefix)
    prefix = prefix or ''
    for k, v in pairs(config) do
      local full_path = prefix == '' and k or (prefix .. '.' .. k)
      local base_path = vim.split(full_path, '.', { plain = true })[1]
      local is_plugin_config = base_path == 'plugins' and type(k) == 'number'

      if
        not (
          (array_paths[base_path] and type(k) == 'number')
          or (dict_paths[base_path] and type(k) == 'string')
          or is_plugin_config
        ) and not is_valid_path(full_path)
      then
        table.insert(warnings, string.format('Unknown configuration entry: `%s`', full_path))
      end

      if dict_paths[base_path] and type(k) == 'string' then
        if not validate_type(v, dict_paths[base_path]) then
          table.insert(errors, {
            msg = string.format('Invalid type \'%s\' for `%s`', type(v), full_path),
            hint = string.format(
              'Allowed types: \'%s\'',
              table.concat(dict_paths[base_path], '\', \'')
            ),
          })
        end
      end

      if type(v) == 'table' and not (skip_subtrees[base_path] and type(k) == 'number') then
        check_unknown_entries(v, full_path)
      end
    end
  end

  check_unknown_entries(user_config)

  for path, allowed_types in pairs(validation_rules) do
    local value = get_nested_value(user_config, path)
    if value ~= nil and not validate_type(value, allowed_types) then
      table.insert(errors, {
        msg = string.format('Invalid type \'%s\' for `%s`', type(value), path),
        hint = string.format('Allowed types: \'%s\'', table.concat(allowed_types, '\', \'')),
      })
    end
  end

  return {
    is_valid = #errors == 0 and #warnings == 0,
    errors = errors,
    warnings = warnings,
  }
end

M.check = function()
  local health = vim.health or require 'cord.api.config'
  local start = health.start or health.report_start
  local ok = health.ok or health.report_ok
  local info = health.info or health.report_info
  local warn = health.warn or health.report_warn
  local err = health.error or health.report_error

  start 'cord.nvim'

  local os_info = vim.loop.os_uname()
  local wsl_info = os.getenv 'WSL_DISTRO_NAME'
  info(
    'System information:\n'
      .. '  Sysname: `'
      .. os_info.sysname
      .. '`\n'
      .. '  Architecture: `'
      .. os_info.machine
      .. '`\n'
      .. '  Release: `'
      .. os_info.release
      .. '`\n'
      .. '  Version: `'
      .. os_info.version
      .. '`'
      .. (wsl_info and ('\n  Running inside WSL (`' .. wsl_info .. '`)') or '')
  )
  info('Neovim version: `' .. tostring(vim.version()) .. '`')
  info('Lua version: `' .. tostring(_VERSION) .. (jit and ' (with LuaJIT)`' or '`'))
  info('Cord connection status: `' .. tostring(require('cord.server').status) .. '`\n')

  if vim.fn.executable 'curl' == 1 then
    ok '`curl` is installed'
  else
    warn '`curl` is not installed or not in PATH'
  end

  local results = M.validate(require('cord').user_config)
  if results.is_valid then
    ok 'No configuration issues found'
  else
    for _, error in ipairs(results.errors) do
      err(error.msg, error.hint)
    end

    for _, warning in ipairs(results.warnings) do
      warn(warning)
    end
  end
end

return M
