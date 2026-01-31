local logger = require 'cord.api.log'
local mappings = require 'cord.internal.activity.mappings'
local icons = require 'cord.api.icon'
local config = require 'cord.api.config'
local async = require 'cord.core.async'

local function get_custom_asset(config, filename, filetype)
  if not config.assets then return end

  local icon = config.assets[filetype]
  if icon then return icon end

  icon = config.assets[filename]
  if icon then return icon end

  local extension = filename:match '%..*$'
  icon = config.assets[extension]
  if icon then return icon end

  icon = config.assets['Cord.override']
  if icon then return icon, 'Cord.override' end
end

local function get_option(option, args)
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
        local result = async.is_async(arg) and arg(args):await() or arg(args)
        logger.trace(
          function() return 'config.get: function variable ${' .. var .. '} = ' .. tostring(result) end
        )
        return tostring(result)
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

  local result = ty == 'function'
      and (async.is_async(option) and option(args):await() or option(args))
      or option
  logger.trace(function() return 'config.get: returning ' .. tostring(type(result)) end)
  return result
end

---@return Activity|boolean
local function build_activity(opts)
  if opts.filetype == '' then
    if opts.filename == '' then
      opts.filename = 'a new file'
      opts.filetype = 'Cord.new'
    else
      opts.filetype = 'Cord.unknown'
    end
  elseif opts.filetype == 'checkhealth' then
    opts.filename = 'checkhealth'
  end

  local icon_type, icon, tooltip = mappings.get(opts.filetype, opts.filename, opts.buftype, opts)
  opts.type = icon_type or 'language'
  opts.icon = icons.get(icon or mappings.get_default_icon(opts.type))
  opts.tooltip = tooltip
  opts.name = tooltip

  local custom_icon, override_type = get_custom_asset(config, opts.filename, opts.filetype)

  if custom_icon then
    if type(custom_icon) == 'string' then
      opts.icon = custom_icon
    else
      opts.name = get_option(custom_icon.name, opts)
      opts.tooltip = get_option(custom_icon.tooltip, opts) or tooltip
      opts.type = custom_icon.type or icon_type
      opts.text = get_option(custom_icon.text, opts)
      opts.filetype = override_type or opts.filetype
      opts.icon = get_option(custom_icon.icon, opts)
          or (custom_icon.type and icons.get(mappings.get_default_icon(custom_icon.type)))
          or opts.icon
    end
  end

  local file_text
  if opts.text then
    file_text = opts.text
  else
    if opts.type == 'language' then
      if opts.is_read_only then
        local text = get_option(config.text.viewing, opts)
        if text == true or text == false then return text end
        if text ~= '' then file_text = text end
      else
        local text = get_option(config.text.editing, opts)
        if text == true or text == false then return text end
        if text ~= '' then file_text = text end
      end
    else
      local text = get_option(config.text[opts.type], opts)
      if text == true or text == false then return text end
      if text ~= '' then file_text = text end
    end
  end

  local details, state
  if config.display.swap_fields then
    local workspace = get_option(config.text.workspace, opts)
    if workspace == true or workspace == false then return workspace end
    if workspace ~= '' then details = workspace end
    state = file_text
  else
    local workspace = get_option(config.text.workspace, opts)
    if workspace == true or workspace == false then return workspace end
    if workspace ~= '' then state = workspace end
    details = file_text
  end

  local large_image, large_text, small_image, small_text

  local function set_editor_only()
    large_image = config.editor.icon
    large_text = get_option(config.editor.tooltip, opts)
  end

  local function set_asset_only()
    large_image = opts.icon
    large_text = opts.tooltip or opts.filetype
  end

  local function set_full()
    if config.display.swap_icons then
      large_image = config.editor.icon
      large_text = get_option(config.editor.tooltip, opts)
      small_image = opts.icon
      small_text = opts.tooltip or opts.filetype
    else
      large_image = opts.icon
      large_text = opts.tooltip or opts.filetype
      small_image = config.editor.icon
      small_text = get_option(config.editor.tooltip, opts)
    end
  end

  if config.display.view == 'auto' then
    if opts.filetype == 'Cord.new' then
      set_editor_only()
    else
      set_full()
    end
  elseif config.display.view == 'editor' then
    set_editor_only()
  elseif config.display.view == 'asset' then
    set_asset_only()
  elseif config.display.view == 'full' or config.display.view == 'auto' then
    set_full()
  end

  return {
    details = details,
    state = state,
    assets = {
      large_image = large_image,
      large_text = large_text,
      small_image = small_image,
      small_text = small_text,
    },
    timestamps = {
      start = opts.timestamp,
    },
    buttons = opts.buttons,
  }
end

---@return Activity
local function build_idle_activity(opts)
  local details, state
  if config.display.swap_fields then
    details = get_option(config.idle.state, opts)
    state = get_option(config.idle.details, opts)
  else
    details = get_option(config.idle.details, opts)
    state = get_option(config.idle.state, opts)
  end

  local large_image, large_text, small_image, small_text

  local function set_editor_only()
    large_image = config.editor.icon
    large_text = get_option(config.editor.tooltip, opts)
  end

  local function set_asset_only()
    large_image = get_option(config.idle.icon, opts)
    large_text = get_option(config.idle.tooltip, opts)
  end

  local function set_full()
    if config.display.swap_icons then
      large_image = config.editor.icon
      large_text = get_option(config.editor.tooltip, opts)
      small_image = get_option(config.idle.icon, opts)
      small_text = get_option(config.idle.tooltip, opts)
    else
      large_image = get_option(config.idle.icon, opts)
      large_text = get_option(config.idle.tooltip, opts)
      small_image = config.editor.icon
      small_text = get_option(config.editor.tooltip, opts)
    end
  end

  if config.display.view == 'editor' then
    set_editor_only()
  elseif config.display.view == 'asset' then
    set_asset_only()
  elseif config.display.view == 'full' or config.display.view == 'auto' then
    set_full()
  end

  return {
    details = details,
    state = state,
    assets = {
      large_image = large_image,
      large_text = large_text,
      small_image = small_image,
      small_text = small_text,
    },
    timestamps = {
      start = opts.timestamp,
    },
    buttons = opts.buttons,
    is_idle = config.idle.smart_idle,
  }
end

return {
  build_activity = build_activity,
  build_idle_activity = build_idle_activity,
}
