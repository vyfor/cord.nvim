local config = require 'cord.plugin.config'

local M = {}

function M.validate(user_config)
  local logger = require 'cord.plugin.log'
  local icons = require 'cord.api.icon'

  local config_manager = require 'cord.plugin.config'
  local final_config = vim.tbl_deep_extend('force', config_manager.get(), user_config or require('cord').user_config or {})

  local log_level = final_config.log_level
  if type(log_level) == 'string' then
    local level = vim.log.levels[string.upper(log_level)]
    if not level then
      logger.log_raw(vim.log.levels.ERROR, 'Unknown log level: ' .. log_level)
      return
    end
    log_level = level
  elseif type(log_level) ~= 'number' then
    logger.log_raw(vim.log.levels.ERROR, 'Log level must be a string or `vim.log.levels.*`')
    return
  end

  final_config.log_level = log_level
  logger.set_level(log_level)
  icons.set(final_config.display.theme, final_config.display.flavor)

  if final_config.buttons then
    if #final_config.buttons > 2 then
      logger.log_raw(vim.log.levels.ERROR, 'There cannot be more than 2 buttons')
      return
    end

    for _, button in ipairs(final_config.buttons) do
      if not button.label or not button.url then
        logger.log_raw(vim.log.levels.ERROR, 'Each button must have a label and a URL')
        return
      end

      if type(button.url) == 'string' and not button.url:match '^https?://[^%s]+$' then
        logger.log_raw(vim.log.levels.ERROR, '`' .. button.url .. '` is not a valid button URL')
        return
      end
    end
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

      logger.log_raw(vim.log.levels.ERROR, 'Unknown client: ' .. final_config.editor.client)
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
  return final_config
end

function M.get(option, args)
  local ty = type(option)

  local variables = config.variables
  local vars_is_table = type(variables) == 'table'
  if vars_is_table then
    ---@cast variables table
    for k, v in pairs(variables) do
      args[k] = v
    end
  end

  if ty == 'string' and (vars_is_table or variables == true) then
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
