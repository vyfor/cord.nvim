local config = require 'cord.plugin.config'

local M = {}

function M:validate(user_config)
  local logger = require 'cord.plugin.log'
  local icons = require 'cord.api.icon'

  local config_manager = require 'cord.plugin.config'
  local final_config = vim.tbl_deep_extend('force', config_manager.get(), user_config)
  logger.set_level(final_config.advanced.plugin.log_level)
  icons.set_theme(final_config.display.theme)

  if final_config.buttons and #final_config.buttons > 2 then
    logger.error 'There cannot be more than 2 buttons'
    return
  end

  if type(final_config.editor.client) == 'string' then
    local client = require('cord.plugin.constants').CLIENT_IDS[final_config.editor.client]

    if not client then
      if final_config.editor.client:match '^%d+$' then
        final_config.is_custom_client = true
        if not final_config.editor.icon then
          final_config.editor.icon = icons.get 'neovim'
        end
        goto continue
      end

      logger.error('Unknown client: ' .. final_config.editor.client)
      return
    end

    final_config.editor.client = client.id
    if not final_config.editor.icon then
      final_config.editor.icon = icons.get(client.icon)
    end
  end

  ::continue::

  if not final_config.idle.icon then
    final_config.idle.icon = icons.get(icons.DEFAULT_IDLE_ICON)
  end

  config_manager.set(final_config)
  self.user_config = user_config

  return final_config
end

function M.get(option, args)
  local ty = type(option)

  local variables = config.variables
  if type(variables) == 'table' then
    for k, v in pairs(variables) do
      args[k] = v
    end
  end

  if ty == 'string' then
    option = option:gsub('%${(.-)}', function(var)
      local arg = args[var]
      if type(arg) == 'function' then
        return tostring(arg(args))
      elseif arg ~= nil then
        return tostring(arg)
      end
      return '${' .. var .. '}'
    end)
  end

  return ty == 'function' and option(args) or option
end

function M.get_buttons(opts)
  if not config.buttons then return {} end

  local buttons = {}
  for i = 1, #config.buttons do
    local sourcebtn = config.buttons[i]
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