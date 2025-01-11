local config_utils = require 'cord.plugin.config.util'
local mappings = require 'cord.plugin.activity.mappings'
local icons = require 'cord.api.icon'
local config = require('cord.plugin.config').opts

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

local text_config = {
  workspace = config.text.workspace,
  viewing = config.text.viewing,
  editing = config.text.editing,
  file_browser = config.text.file_browser,
  plugin_manager = config.text.plugin_manager,
  lsp = config.text.lsp,
  docs = config.text.docs,
  vcs = config.text.vcs,
  notes = config.text.notes,
  debug = config.text.debug,
  test = config.text.test,
  diagnostics = config.text.diagnostics,
  dashboard = config.text.dashboard,
}

local function build_activity(opts)
  if opts.filetype == '' then
    if opts.filename == '' then
      opts.filename = 'a new file'
      opts.filetype = 'Cord.new'
    else
      opts.filetype = 'Cord.unknown'
    end
  end

  local icon_type, icon, tooltip = mappings.get(opts.filetype, opts.filename)
  opts.type = icon_type or 'language'
  opts.icon = icons.get(icon or mappings.get_default_icon(opts.type))
  opts.tooltip = tooltip

  local custom_icon, override_type = get_custom_asset(config, opts.filename, opts.filetype)

  if custom_icon then
    if type(custom_icon) == 'string' then
      opts.icon = custom_icon
    else
      opts.name = config_utils:get(custom_icon.name, opts)
      opts.tooltip = config_utils:get(custom_icon.tooltip, opts) or tooltip
      opts.type = custom_icon.type or icon_type
      opts.text = config_utils:get(custom_icon.text, opts)
      opts.filetype = override_type or opts.filetype
      opts.icon = config_utils:get(custom_icon.icon, opts) or icon
    end
  end

  local file_text
  if opts.text then
    file_text = opts.text
  else
    if opts.type == 'language' then
      if opts.is_read_only then
        file_text = config_utils:get(text_config.viewing, opts)
      else
        file_text = config_utils:get(text_config.editing, opts)
      end
    else
      file_text = config_utils:get(text_config[opts.type], opts)
    end
  end

  local details, state
  if config.display.swap_fields then
    details = config_utils:get(text_config.workspace, opts)
    state = file_text
  else
    details = file_text
    state = config_utils:get(text_config.workspace, opts)
  end

  local large_image, large_text, small_image, small_text
  if opts.filetype == 'Cord.new' then
    large_image = config.editor.icon
    large_text = config_utils:get(config.editor.tooltip, opts)
  elseif config.display.swap_icons then
    large_image = config.editor.icon
    large_text = config_utils:get(config.editor.tooltip, opts)
    small_image = opts.icon
    small_text = opts.tooltip or opts.filetype
  else
    large_image = opts.icon
    large_text = opts.tooltip or opts.filetype
    small_image = config.editor.icon
    small_text = config_utils:get(config.editor.tooltip, opts)
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

local function build_idle_activity(opts)
  local details, state
  if config.display.swap_fields then
    details = config_utils:get(config.idle.state, opts)
    state = config_utils:get(config.idle.details, opts)
  else
    details = config_utils:get(config.idle.details, opts)
    state = config_utils:get(config.idle.state, opts)
  end

  local large_image, large_text, small_image, small_text
  if config.display.swap_icons then
    large_image = config.editor.icon
    large_text = config_utils:get(config.editor.tooltip, opts)
    small_image = config_utils:get(config.idle.icon, opts)
    small_text = config_utils:get(config.idle.tooltip, opts)
  else
    large_image = config_utils:get(config.idle.icon, opts)
    large_text = config_utils:get(config.idle.tooltip, opts)
    small_image = config.editor.icon
    small_text = config_utils:get(config.editor.tooltip, opts)
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
