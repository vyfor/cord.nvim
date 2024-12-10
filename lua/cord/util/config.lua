local logger = require 'cord.util.logger'
local constants = require 'cord.util.constants'
local utils = require 'cord.util'

local M = {}

M.values = {
  usercmds = true,
  log_level = vim.log.levels.ERROR,
  timestamp = {
    enabled = true,
    reset_on_idle = false,
    reset_on_change = false,
  },
  editor = {
    client = 'neovim',
    tooltip = 'The Superior Text Editor',
    icon = nil,
  },
  display = {
    swap_fields = false,
    swap_icons = false,
  },
  lsp = {
    severity = vim.diagnostic.severity.ERROR,
    scope = 'workspace',
  },
  idle = {
    enabled = true,
    timeout = 300000,
    show_status = true,
    ignore_focus = true,
    smart_idle = true,
    details = 'Idling',
    state = nil,
    tooltip = '💤',
    icon = nil,
  },
  text = {
    viewing = function(opts)
      return 'Viewing '
        .. (opts.filename ~= '' and opts.filename or 'a new file')
    end,
    editing = function(opts)
      return 'Editing '
        .. (opts.filename ~= '' and opts.filename or 'a new file')
    end,
    file_browser = function(opts) return 'Browsing files in ' .. opts.tooltip end,
    plugin_manager = function(opts)
      return 'Managing plugins in ' .. opts.tooltip
    end,
    lsp_manager = function(opts) return 'Configuring LSP in ' .. opts.tooltip end,
    docs = function(opts) return 'Reading ' .. opts.tooltip end,
    vcs = function(opts) return 'Committing changes in ' .. opts.tooltip end,
    workspace = function(opts) return 'In ' .. opts.workspace_name end,
    dashboard = 'Home',
  },
  buttons = nil,
  assets = nil,
  hooks = {
    on_ready = nil,
    on_update = nil,
    on_activity = nil,
    on_idle = nil,
    on_workspace_change = nil,
    on_disconnect = nil,
  },
  advanced = {
    server = {
      pipe_path = nil,
      executable_path = nil,
      timeout = 60000,
    },
    cursor_update_mode = 'on_move',
  },
}

function M:validate(user_config)
  local config = vim.tbl_deep_extend('force', self.values, user_config)

  if config.buttons and #config.buttons > 2 then
    logger.error 'config.buttons cannot have more than 2 buttons'
    return false
  end

  if type(config.editor.client) == 'string' then
    local client = constants.CLIENT_IDS[config.editor.client]

    if not client then
      if config.editor.client:match '^%d+$' then
        config.is_custom_client = true
        if not config.editor.icon then
          config.editor.icon = utils.get_asset('editor', 'neovim')
        end
        goto continue
      end

      logger.error('Unknown client: ' .. config.editor.client)
      return false
    end

    config.editor.client = client.id
    config.editor.icon = utils.get_asset('editor', client.icon)
  end

  ::continue::

  self.values = config

  return true
end

function M.get(option, args)
  if type(option) == 'function' then return option(args) end

  return option
end

function M:get_buttons()
  local buttons = self.values.buttons
  if not buttons then return end

  for i = 1, #buttons do
    local button = buttons[i]

    if type(button.label) == 'function' then
      buttons[i].label = button.label(self.values)
    end

    if type(button.url) == 'function' then
      buttons[i].url = button.url(self.values)
    end
  end

  return buttons
end

return M
