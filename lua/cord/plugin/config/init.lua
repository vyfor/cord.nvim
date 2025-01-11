---@class CordTimestampConfig
---@field enabled? boolean Whether timestamps are enabled
---@field reset_on_idle? boolean Whether to reset timestamp when idle
---@field reset_on_change? boolean Whether to reset timestamp when changing activities

---@class CordEditorConfig
---@field client? string Editor client name, one of 'vim', 'neovim', 'lunarvim', 'nvchad', 'astronvim' or a custom Discord application ID
---@field tooltip? string Editor tooltip text
---@field icon? string Optional editor icon

---@class CordDisplayConfig
---@field theme? string Set icon theme
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
---@field icon? string Idle icon

---@class CordTextConfig
---@field workspace? string|fun(opts: CordOpts):string Text for workspace activity
---@field viewing? string|fun(opts: CordOpts):string Text for viewing activity
---@field editing? string|fun(opts: CordOpts):string Text for editing activity
---@field file_browser? string|fun(opts: CordOpts):string Text for file browser activity
---@field plugin_manager? string|fun(opts: CordOpts):string Text for plugin manager activity
---@field lsp? string|fun(opts: CordOpts):string Text for LSP manager activity
---@field docs? string|fun(opts: CordOpts):string Text for documentation activity
---@field vcs? string|fun(opts: CordOpts):string Text for VCS activity
---@field notes? string|fun(opts: CordOpts):string Text for notes activity
---@field debug? string|fun(opts: CordOpts):string Text for debugging-related plugin activity
---@field test? string|fun(opts: CordOpts):string Text for testing-related plugin activity
---@field games? string|fun(opts: CordOpts):string Text for games activity
---@field diagnostics? string|fun(opts: CordOpts):string Text for diagnostics activity
---@field dashboard? string|fun(opts: CordOpts):string Text for dashboard activity

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
---@field on_ready? fun(manager: ActivityManager):nil
---@field on_update? fun(opts: CordOpts):nil
---@field on_activity? fun(opts: CordOpts, activity: Activity):nil
---@field on_idle? fun(opts: CordOpts, activity: Activity?):nil
---@field on_workspace_change? fun(opts: CordOpts):nil
---@field on_disconnect? fun():nil

---@class CordAdvancedConfig
---@field plugin? CordAdvancedPluginConfig configuration
---@field server? CordAdvancedServerConfig configuration
---@field discord? CordAdvancedDiscordConfig configuration

---@class CordAdvancedPluginConfig
---@field autocmds? boolean Whether to enable autocmds
---@field log_level? integer Logging level (from `vim.log.levels`)
---@field cursor_update? string Cursor update mode
---@field variables_in_functions? boolean Whether to use variables in functions
---@field match_in_mappings? boolean Whether to match against file extensions in mappings

---@class CordAdvancedServerConfig
---@field update? string How to acquire the server executable: 'fetch' or 'build' or 'none'
---@field pipe_path? string Path to the server's pipe
---@field executable_path? string Path to the server's executable
---@field timeout? integer Timeout in milliseconds

---@class CordAdvancedDiscordConfig
---@field reconnect? CordAdvancedDiscordReconnectConfig Reconnection configuration

---@class CordAdvancedDiscordReconnectConfig
---@field enabled? boolean Whether reconnection is enabled
---@field interval? integer Reconnection interval in milliseconds, 0 to disable

---@alias CordVariablesConfig { [string]: string|fun(opts: CordOpts):string }

---@class CordConfig
---@field editor? CordEditorConfig Editor configuration
---@field display? CordDisplayConfig Display configuration
---@field timestamp? CordTimestampConfig Timestamp configuration
---@field idle? CordIdleConfig Idle configuration
---@field text? CordTextConfig Text configuration
---@field buttons? CordButtonConfig[] Buttons configuration
---@field assets? CordAssetConfig[] Assets configuration
---@field variables? boolean|CordVariablesConfig Variables configuration. If true, uses default options table. If table, extends default table. If false, disables custom variables.
---@field hooks? CordHooksConfig Hooks configuration
---@field advanced? CordAdvancedConfig Advanced configuration

local M = {}

---@type CordConfig
M.opts = {
  editor = {
    client = 'neovim',
    tooltip = 'The Superior Text Editor',
    icon = nil,
  },
  display = {
    theme = 'onyx',
    swap_fields = false,
    swap_icons = false,
  },
  timestamp = {
    enabled = true,
    reset_on_idle = false,
    reset_on_change = false,
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
    workspace = function(opts) return 'In ' .. opts.workspace_name end,
    viewing = function(opts) return 'Viewing ' .. opts.filename end,
    editing = function(opts) return 'Editing ' .. opts.filename end,
    file_browser = function(opts) return 'Browsing files in ' .. opts.tooltip end,
    plugin_manager = function(opts) return 'Managing plugins in ' .. opts.tooltip end,
    lsp = function(opts) return 'Configuring LSP in ' .. opts.tooltip end,
    docs = function(opts) return 'Reading ' .. opts.tooltip end,
    vcs = function(opts) return 'Committing changes in ' .. opts.tooltip end,
    notes = function(opts) return 'Taking notes in ' .. opts.tooltip end,
    debug = function(opts) return 'Debugging in ' .. opts.tooltip end,
    test = function(opts) return 'Testing in ' .. opts.tooltip end,
    diagnostics = function(opts) return 'Fixing problems in ' .. opts.tooltip end,
    games = function(opts) return 'Playing ' .. opts.tooltip end,
    dashboard = 'Home',
  },
  buttons = nil,
  assets = nil,
  variables = nil,
  hooks = {
    on_ready = nil,
    on_update = nil,
    on_activity = nil,
    on_idle = nil,
    on_workspace_change = nil,
    on_disconnect = nil,
  },
  advanced = {
    plugin = {
      autocmds = true,
      log_level = vim.log.levels.INFO,
      cursor_update = 'on_hold',
      variables_in_functions = false,
      match_in_mappings = true,
    },
    server = {
      update = 'fetch',
      pipe_path = nil,
      executable_path = nil,
      timeout = 300000,
    },
    discord = {
      reconnect = {
        enabled = false,
        interval = 5000,
      },
    },
  },
}

M.set_config = function(config) M.opts = config end

return M
