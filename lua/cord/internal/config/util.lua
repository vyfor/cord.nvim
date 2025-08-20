local config = require 'cord.internal.config'
local logger = require 'cord.api.log'

local M = {}

function M.validate(new_config)
  local user_config = new_config or require('cord').user_config or {}
  local icons = require 'cord.api.icon'
  local config_manager = require 'cord.internal.config'

  local final_config = vim.tbl_deep_extend('force', config_manager.get(), user_config)

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

  if user_config.text and user_config.text.default then
    local default_text = user_config.text.default
    for key, _ in pairs(final_config.text) do
      if key ~= 'default' and not user_config.text[key] then
        final_config.text[key] = default_text
      end
    end
  end

  config_manager.set(final_config)
  return final_config
end

function M.get(option, args)
  local ty = type(option)
  logger.trace(function() return 'config.get: option_type=' .. tostring(ty) end)

  local variables = config.variables
  local vars_is_table = type(variables) == 'table'
  if vars_is_table then
    ---@cast variables table
    logger.trace(function()
      local keys = {}
      for k, _ in pairs(variables) do
        table.insert(keys, k)
      end
      return 'config.get: merging variables=[' .. table.concat(keys, ',') .. ']'
    end)
    for k, v in pairs(variables) do
      args[k] = v
    end
  end

  if ty == 'string' and (vars_is_table or variables == true) then
    logger.trace(
      function() return 'config.get: processing string with variables: ' .. tostring(option) end
    )
    option = option:gsub('%${(.-)}', function(var)
      local arg = args[var]
      logger.trace(
        function() return 'config.get: variable ${' .. var .. '} = ' .. tostring(arg) end
      )
      if type(arg) == 'function' then
        local result = tostring(arg(args))
        logger.trace(
          function() return 'config.get: function variable ${' .. var .. '} = ' .. result end
        )
        return result
      elseif arg ~= nil then
        return tostring(arg)
      end
      logger.trace(
        function() return 'config.get: undefined variable ${' .. var .. '}, keeping placeholder' end
      )
      return '${' .. var .. '}'
    end)
    logger.trace(function() return 'config.get: final string: ' .. tostring(option) end)
  end

  local result = ty == 'function' and option(args) or option
  logger.trace(function() return 'config.get: returning ' .. tostring(type(result)) end)
  return result
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
