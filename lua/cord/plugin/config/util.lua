local M = {}

function M:validate(user_config)
  local logger = require 'cord.plugin.log'
  local icons = require 'cord.api.icon'

  local config_manager = require 'cord.plugin.config'
  local config = vim.tbl_deep_extend('force', config_manager.opts, user_config)
  logger.set_level(config.advanced.plugin.log_level)
  icons.set_theme(config.display.theme)

  if config.buttons and #config.buttons > 2 then
    logger.error 'There cannot be more than 2 buttons'
    return
  end

  if type(config.editor.client) == 'string' then
    local client = require('cord.plugin.constants').CLIENT_IDS[config.editor.client]

    if not client then
      if config.editor.client:match '^%d+$' then
        config.is_custom_client = true
        if not config.editor.icon then
          config.editor.icon = icons.get 'neovim'
        end
        goto continue
      end

      logger.error('Unknown client: ' .. config.editor.client)
      return
    end

    config.editor.client = client.id
    if not config.editor.icon then
      config.editor.icon = icons.get(client.icon)
    end
  end

  ::continue::

  if not config.idle.icon then
    config.idle.icon = icons.get(icons.DEFAULT_IDLE_ICON)
  end

  config_manager.set_config(config)
  self.config = config

  return config
end

function M:get(option, args)
  local is_function = type(option) == 'function'

  if is_function then
    if not self.config.advanced.plugin.variables_in_functions then return option(args) end
  else
    local variables = self.config.variables
    if variables then
      if type(variables) == 'table' then
        for k, v in pairs(variables) do
          args[k] = (type(v) == 'function') and v(args) or v
        end
      end
      if type(option) == 'string' then
        option = option:gsub('%${(.-)}', args)
      end
    end
  end

  return is_function and option(args) or option
end

function M:get_buttons(opts)
  if not self.config.buttons then return {} end

  local buttons = {}
  for i = 1, #self.config.buttons do
    local sourcebtn = self.config.buttons[i]
    local button = {}

    if type(sourcebtn.label) == 'function' then
      local label = sourcebtn.label(opts)
      if not label then goto continue end
      button.label = label
    else
      if not sourcebtn.label then goto continue end
      button.label = sourcebtn.label
    end

    if type(sourcebtn.url) == 'function' then
      local url = sourcebtn.url(opts)
      if not url then goto continue end
      button.url = url
    else
      if not sourcebtn.url then goto continue end
      button.url = sourcebtn.url
    end

    buttons[#buttons + 1] = button

    ::continue::
  end

  return buttons
end

return M