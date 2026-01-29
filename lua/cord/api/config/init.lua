local utils = require 'cord.api.config.util'
local logger = require 'cord.api.log'

---@class CordTimestampConfig
---@field enabled? boolean Whether timestamps are enabled
---@field reset_on_idle? boolean Whether to reset timestamp when idle
---@field reset_on_change? boolean Whether to reset timestamp when changing activities
---@field shared? boolean Whether to share timestamps between clients

---@class CordEditorConfig
---@field client? 'vim'|'neovim'|'lunarvim'|'nvchad'|'astronvim'|'lazyvim'|string Editor client name, one of 'vim', 'neovim', 'lunarvim', 'nvchad', 'astronvim', 'lazyvim' or a custom Discord application ID
---@field tooltip? string Editor tooltip text
---@field icon? string Optional editor icon

---@class CordDisplayConfig
---@field theme? 'default'|'atom'|'catppuccin'|'minecraft'|'classic'|string Set icon theme
---@field flavor? 'dark'|'light'|'accent'|string Set icon theme flavor
---@field view? 'full'|'editor'|'asset'|'auto'|string Control what shows up as the large and small images
---@field swap_fields? boolean Whether to swap activity fields
---@field swap_icons? boolean Whether to swap activity icons

---@class CordIdleConfig
---@field enabled? boolean Whether idle detection is enabled
---@field timeout? integer Idle timeout in milliseconds
---@field show_status? boolean Whether to show idle status
---@field ignore_focus? boolean Whether to show idle when editor is focused
---@field unidle_on_focus? boolean Whether to unidle the session when editor gains focus
---@field smart_idle? boolean Whether to enable smart idle feature
---@field details? string|fun(opts: CordOpts):string Details shown when idle
---@field state? string|fun(opts: CordOpts):string State shown when idle
---@field tooltip? string|fun(opts: CordOpts):string Tooltip shown when hovering over idle icon
---@field icon? string|fun(opts: CordOpts):string Idle icon

---@class CordTextConfig
---@field default? string|fun(opts: CordOpts):string|boolean|nil Default text for all activities
---@field workspace? string|fun(opts: CordOpts):string|boolean|nil Text for workspace activity
---@field viewing? string|fun(opts: CordOpts):string|boolean|nil Text for viewing activity
---@field editing? string|fun(opts: CordOpts):string|boolean|nil Text for editing activity
---@field file_browser? string|fun(opts: CordOpts):string|boolean|nil Text for file browser activity
---@field plugin_manager? string|fun(opts: CordOpts):string|boolean|nil Text for plugin manager activity
---@field lsp? string|fun(opts: CordOpts):string|boolean|nil Text for LSP manager activity
---@field docs? string|fun(opts: CordOpts):string|boolean|nil Text for documentation activity
---@field vcs? string|fun(opts: CordOpts):string|boolean|nil Text for VCS activity
---@field notes? string|fun(opts: CordOpts):string|boolean|nil Text for notes activity
---@field debug? string|fun(opts: CordOpts):string|boolean|nil Text for debugging-related plugin activity
---@field test? string|fun(opts: CordOpts):string|boolean|nil Text for testing-related plugin activity
---@field games? string|fun(opts: CordOpts):string|boolean|nil Text for games activity
---@field diagnostics? string|fun(opts: CordOpts):string|boolean|nil Text for diagnostics activity
---@field terminal? string|fun(opts: CordOpts):string|boolean|nil Text for terminal activity
---@field dashboard? string|fun(opts: CordOpts):string|boolean|nil Text for dashboard activity

---@class CordButtonConfig
---@field label string|fun(opts: CordOpts):string? Button label
---@field url string|fun(opts: CordOpts):string? Button URL

---@class CordAssetConfig
---@field name? string|fun(opts: CordOpts):string Asset name
---@field icon? string|fun(opts: CordOpts):string Asset icon
---@field tooltip? string|fun(opts: CordOpts):string Asset tooltip
---@field text? string|fun(opts: CordOpts):string Asset text
---@field type? string|fun(opts: CordOpts):string Asset type

---@class CordHooksConfig
---@field ready? CordManagerHook
---@field shutdown? CordEmptyHook
---@field pre_activity? CordHook
---@field post_activity? CordActivityHook
---@field idle_enter? CordHook
---@field idle_leave? CordHook
---@field workspace_change? CordHook
---@field buf_enter? CordManagerHook

---@alias CordHook fun(opts: CordOpts):nil | {fun: fun(opts: CordOpts):nil, priority: number}
---@alias CordManagerHook fun(manager: ActivityManager):nil | {fun: fun(manager: ActivityManager):nil, priority: number}
---@alias CordEmptyHook fun():nil | {fun: fun():nil, priority: number}
---@alias CordActivityHook fun(opts: CordOpts, activity: Activity):nil | {fun: fun(opts: CordOpts, activity: Activity):nil, priority: number}

---@class CordAdvancedConfig
---@field plugin? CordAdvancedPluginConfig configuration
---@field server? CordAdvancedServerConfig configuration
---@field discord? CordAdvancedDiscordConfig configuration
---@field workspace? CordAdvancedWorkspaceConfig configuration

---@class CordAdvancedWorkspaceConfig
---@field root_markers? string[] Root markers to use for finding workspaces
---@field limit_to_cwd? boolean Whether to limit workspace detection to the working directory (vim.fn.getcwd()). When true, workspace detection stops at the CWD if no marker is found.

---@class CordAdvancedPluginConfig
---@field autocmds? boolean Whether to enable autocmds
---@field cursor_update? string Cursor update mode
---@field match_in_mappings? boolean Whether to match against file extensions in mappings
---@field debounce? CordAdvancedDebounceConfig Debounce/throttle configuration for activity updates

---@class CordAdvancedDebounceConfig
---@field delay? integer Delay in milliseconds before sending the first update. Allows events received in quick succession (e.g., buffer switches) to settle before sending data. Set to 0 to disable.
---@field interval? integer Minimum interval in milliseconds between updates. Prevents flooding the server during rapid cursor movement. Set to 0 to disable.

---@class CordAdvancedServerConfig
---@field update? 'fetch'|'install'|'build'|'none'|string How to acquire the server executable: 'fetch' or 'install' or 'build' or 'none'
---@field pipe_path? string Path to the server's pipe
---@field executable_path? string Path to the server's executable
---@field timeout? integer Timeout in milliseconds

---@class CordAdvancedDiscordConfig
---@field pipe_paths? string[] Custom IPC pipe paths to use when connecting to Discord
---@field reconnect? CordAdvancedDiscordReconnectConfig Reconnection settings
---@field sync? CordAdvancedSyncConfig Synchronization settings

---@class CordAdvancedDiscordReconnectConfig
---@field enabled? boolean Whether reconnection is enabled
---@field interval? integer Reconnection interval in milliseconds, 0 to disable
---@field initial? boolean Whether to reconnect if initial connection fails

---@class CordAdvancedSyncConfig
---@field enabled? boolean Whether synchronization logic is enabled
---@field mode? 'periodic'|'defer' Synchronization mode
---@field interval? integer Interval in milliseconds
---@field reset_on_update? boolean Whether to reset periodic synchronization on activity updates
---@field pad? boolean Whether to pad activity fields

---@alias CordVariablesConfig { [string]: string|fun(opts: CordOpts):string }

---@class CordConfig
---@field enabled? boolean Whether Cord plugin is enabled
---@field log_level? string|integer Log level (from `vim.log.levels`)
---@field editor? CordEditorConfig Editor configuration
---@field display? CordDisplayConfig Display configuration
---@field timestamp? CordTimestampConfig Timestamp configuration
---@field idle? CordIdleConfig Idle configuration
---@field text? CordTextConfig Text configuration
---@field buttons? CordButtonConfig[] Buttons configuration
---@field assets? CordAssetConfig[] Assets configuration
---@field variables? boolean|CordVariablesConfig Variables configuration. If true, uses default options table. If table, extends default table. If false, disables custom variables.
---@field hooks? CordHooksConfig Hooks configuration
---@field plugins? string[]|table<string, table>[] Plugin configuration
---@field advanced? CordAdvancedConfig Advanced configuration

---@class InternalCordConfig: CordConfig
local M = {}

---@type CordConfig
local defaults = {
  enabled = true,
  log_level = vim.log.levels.OFF,
  editor = {
    client = 'neovim',
    tooltip = 'The Superior Text Editor',
    icon = nil,
  },
  display = {
    theme = 'default',
    flavor = 'dark',
    view = 'full',
    swap_fields = false,
    swap_icons = false,
  },
  timestamp = {
    enabled = true,
    reset_on_idle = false,
    reset_on_change = false,
    shared = false,
  },
  idle = {
    enabled = true,
    timeout = 300000,
    show_status = true,
    ignore_focus = true,
    unidle_on_focus = true,
    smart_idle = true,
    details = 'Idling',
    state = nil,
    tooltip = '💤',
    icon = nil,
  },
  text = {
    default = nil,
    workspace = function(opts) return 'In ' .. opts.workspace end,
    viewing = function(opts) return 'Viewing ' .. opts.filename end,
    editing = function(opts) return 'Editing ' .. opts.filename end,
    file_browser = function(opts) return 'Browsing files in ' .. opts.name end,
    plugin_manager = function(opts) return 'Managing plugins in ' .. opts.name end,
    lsp = function(opts) return 'Configuring LSP in ' .. opts.name end,
    docs = function(opts) return 'Reading ' .. opts.name end,
    vcs = function(opts) return 'Committing changes in ' .. opts.name end,
    notes = function(opts) return 'Taking notes in ' .. opts.name end,
    debug = function(opts) return 'Debugging in ' .. opts.name end,
    test = function(opts) return 'Testing in ' .. opts.name end,
    diagnostics = function(opts) return 'Fixing problems in ' .. opts.name end,
    games = function(opts) return 'Playing ' .. opts.name end,
    terminal = function(opts) return 'Running commands in ' .. opts.name end,
    dashboard = 'Home',
  },
  buttons = nil,
  assets = nil,
  variables = nil,
  hooks = {
    ready = nil,
    shutdown = nil,
    pre_activity = nil,
    post_activity = nil,
    idle_enter = nil,
    idle_leave = nil,
    workspace_change = nil,
    buf_enter = nil,
  },
  plugins = nil,
  advanced = {
    plugin = {
      autocmds = true,
      cursor_update = 'on_hold',
      match_in_mappings = true,
      debounce = {
        delay = 50,
        interval = 750,
      },
    },
    server = {
      update = 'fetch',
      pipe_path = nil,
      executable_path = nil,
      timeout = 300000,
    },
    discord = {
      pipe_paths = nil,
      reconnect = {
        enabled = false,
        interval = 5000,
        initial = true,
      },
      sync = {
        enabled = true,
        mode = 'periodic',
        interval = 12000,
        reset_on_update = true,
        pad = true,
      },
    },
    workspace = {
      root_markers = {
        '.git',
        '.hg',
        '.svn',
      },
      limit_to_cwd = false,
    },
  },
}

M.get = function() return defaults end
M.set = function(config) defaults = config end

function M.verify(new_config)
  local user_config = new_config or require('cord').user_config or {}
  local icons = require 'cord.api.icon'

  local final_config = vim.tbl_deep_extend('force', M.get(), user_config)

  local log_level = final_config.log_level
  if type(log_level) == 'string' then
    local level = vim.log.levels[string.upper(log_level)]
    if not level then
      logger.notify('Unknown log level: ' .. log_level, vim.log.levels.ERROR)
      return
    end
    log_level = level
  elseif type(log_level) ~= 'number' then
    logger.notify('Log level must be a string or `vim.log.levels.*`', vim.log.levels.ERROR)
    return
  end

  final_config.log_level = log_level
  logger.set_level(log_level)
  icons.set(final_config.display.theme, final_config.display.flavor)

  if not vim.tbl_contains({ 'auto', 'editor', 'asset', 'full' }, final_config.display.view) then
    logger.notify('View must be one of `auto`, `editor`, `asset`, or `full`', vim.log.levels.ERROR)
    return
  end

  if final_config.buttons then
    if #final_config.buttons > 2 then
      logger.notify('There cannot be more than 2 buttons', vim.log.levels.ERROR)
      return
    end

    for _, button in ipairs(final_config.buttons) do
      if not button.label or not button.url then
        logger.notify('Each button must have a label and a URL', vim.log.levels.ERROR)
        return
      end

      if type(button.url) == 'string' and not button.url:match '^https?://[^%s]+$' then
        logger.notify('`' .. button.url .. '` is not a valid button URL', vim.log.levels.ERROR)
        return
      end
    end
  end

  if type(final_config.editor.client) == 'string' then
    local client = require('cord.internal.constants').CLIENT_IDS[final_config.editor.client]

    if not client then
      if final_config.editor.client:match '^%d+$' then
        final_config.is_custom_client = true
        if not final_config.editor.icon then final_config.editor.icon = icons.get 'neovim' end
        goto continue
      end

      logger.notify('Unknown client: ' .. final_config.editor.client, vim.log.levels.ERROR)
      return
    end

    final_config.editor.client = client.id
    if not final_config.editor.icon then final_config.editor.icon = icons.get(client.icon) end
  end

  ::continue::

  if not final_config.idle.icon then final_config.idle.icon = icons.get(icons.DEFAULT_IDLE_ICON) end

  if final_config.advanced.discord.sync.enabled then
    if not vim.tbl_contains({ 'periodic', 'defer' }, final_config.advanced.discord.sync.mode) then
      logger.notify('Sync mode must be either `periodic` or `defer`', vim.log.levels.ERROR)
      return
    end
  end

  if user_config.text and user_config.text.default then
    local default_text = user_config.text.default
    for key, _ in pairs(final_config.text) do
      if key ~= 'default' and not user_config.text[key] then
        final_config.text[key] = default_text
      end
    end
  end

  M.set(final_config)
  return final_config
end

local rules = {
  fields = {
    ['enabled'] = { 'boolean' },
    ['log_level'] = { 'string', 'number' },

    ['editor'] = { 'table' },
    ['editor.client'] = { 'string' },
    ['editor.tooltip'] = { 'string' },
    ['editor.icon'] = { 'string' },

    ['display'] = { 'table' },
    ['display.theme'] = { 'string' },
    ['display.flavor'] = { 'string' },
    ['display.view'] = { 'string' },
    ['display.swap_fields'] = { 'boolean' },
    ['display.swap_icons'] = { 'boolean' },

    ['timestamp'] = { 'table' },
    ['timestamp.enabled'] = { 'boolean' },
    ['timestamp.reset_on_idle'] = { 'boolean' },
    ['timestamp.reset_on_change'] = { 'boolean' },
    ['timestamp.shared'] = { 'boolean' },

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
    ['text.default'] = { 'string', 'boolean', 'function' },
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
    ['advanced.plugin.debounce'] = { 'table' },
    ['advanced.plugin.debounce.delay'] = { 'number' },
    ['advanced.plugin.debounce.interval'] = { 'number' },
    ['advanced.server'] = { 'table' },
    ['advanced.server.update'] = { 'string' },
    ['advanced.server.pipe_path'] = { 'string' },
    ['advanced.server.executable_path'] = { 'string' },
    ['advanced.server.timeout'] = { 'number' },
    ['advanced.discord'] = { 'table' },
    ['advanced.discord.pipe_paths'] = { 'table' },
    ['advanced.discord.reconnect'] = { 'table' },
    ['advanced.discord.reconnect.enabled'] = { 'boolean' },
    ['advanced.discord.reconnect.interval'] = { 'number' },
    ['advanced.discord.reconnect.initial'] = { 'boolean', 'table' },
    ['advanced.discord.sync'] = { 'table' },
    ['advanced.discord.sync.enabled'] = { 'boolean' },
    ['advanced.discord.sync.mode'] = { 'string' },
    ['advanced.discord.sync.interval'] = { 'number' },
    ['advanced.discord.sync.reset_on_update'] = { 'boolean' },
    ['advanced.discord.sync.pad'] = { 'boolean' },
    ['advanced.workspace'] = { 'table' },
    ['advanced.workspace.root_markers'] = { 'table' },
    ['advanced.workspace.limit_to_cwd'] = { 'boolean' },
  },
  array_paths = {
    ['buttons'] = true,
    ['plugins'] = true,
    ['advanced.discord.pipe_paths'] = true,
    ['advanced.workspace.root_markers'] = true,
  },
  skip_subtrees = {
    ['plugins'] = true,
  },
  dict_paths = {
    ['assets'] = { 'string', 'table' },
  },
}

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
            (rules.array_paths[prefix] and type(k) == 'number')
            or (rules.array_paths[base_path] and type(k) == 'number')
            or (rules.dict_paths[base_path] and type(k) == 'string')
            or is_plugin_config
          ) and not utils.is_valid_path(rules.fields, rules.dict_paths, full_path)
      then
        table.insert(warnings, string.format('Unknown configuration entry: `%s`', full_path))
      end

      if rules.dict_paths[base_path] and type(k) == 'string' then
        if not utils.validate_type(v, rules.dict_paths[base_path]) then
          table.insert(errors, {
            msg = string.format('Invalid type \'%s\' for `%s`', type(v), full_path),
            hint = string.format(
              'Allowed types: \'%s\'',
              table.concat(rules.dict_paths[base_path], '\', \'')
            ),
          })
        end
      end

      if type(v) == 'table' and not (rules.skip_subtrees[base_path] and type(k) == 'number') then
        check_unknown_entries(v, full_path)
      end
    end
  end

  check_unknown_entries(user_config)

  for path, allowed_types in pairs(rules.fields) do
    local value = utils.get_nested_value(user_config, path)
    if value ~= nil and not utils.validate_type(value, allowed_types) then
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
  local health = vim.health
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
  info('Cord connection status: `' .. require('cord.api.command').status(true) .. '`\n')

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

return setmetatable(M, {
  __index = function(_, key) return defaults[key] end,
})
