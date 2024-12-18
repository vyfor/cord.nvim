local config = require 'cord.util.config'
local mappings = require 'cord.mappings'
local utils = require 'cord.util'
local icons = require 'cord.icon'

local function build_activity(cfg, opts)
  if opts.filetype == '' then
    if opts.filename == '' then
      opts.filename = 'a new file'
      opts.filetype = 'Cord.new'
    else
      opts.filetype = 'Cord.unknown'
    end
  end

  local icon_type, icon, tooltip = mappings.get(opts.filetype, opts.filename)
  opts.type = icon_type
  opts.icon = icon and icons.get(icon)
  opts.tooltip = tooltip

  local custom_icon, override_type =
    utils.get_custom_asset(cfg, opts.filename, opts.filetype)
  if custom_icon then
    if type(custom_icon) == 'string' then
      opts.icon = custom_icon
    else
      opts.name = config.get(custom_icon.name, opts)
      opts.tooltip = config.get(custom_icon.tooltip, opts) or tooltip
      opts.type = custom_icon.type or icon_type or 'language'
      opts.text = config.get(custom_icon.text, opts)
      opts.filetype = override_type or opts.filetype
      opts.icon = config.get(custom_icon.icon, opts)
        or icon
        or mappings.get_default_icon(opts.type)
    end
  end

  local file_text
  if opts.text then
    file_text = opts.text
  else
    if icon_type == 'language' then
      if opts.is_read_only then
        file_text = config.get(cfg.text.viewing, opts)
      else
        file_text = config.get(cfg.text.editing, opts)
      end
    elseif icon_type == 'file_browser' then
      file_text = config.get(cfg.text.file_browser, opts)
    elseif icon_type == 'plugin_manager' then
      file_text = config.get(cfg.text.plugin_manager, opts)
    elseif icon_type == 'lsp' then
      file_text = config.get(cfg.text.lsp, opts)
    elseif icon_type == 'docs' then
      file_text = config.get(cfg.text.docs, opts)
    elseif icon_type == 'vcs' then
      file_text = config.get(cfg.text.vcs, opts)
    elseif icon_type == 'dashboard' then
      file_text = config.get(cfg.text.dashboard, opts)
    end
  end

  local details, state
  if cfg.display.swap_fields then
    details = config.get(cfg.text.workspace, opts)
    state = file_text
  else
    details = file_text
    state = config.get(cfg.text.workspace, opts)
  end

  local large_image, large_text, small_image, small_text
  if opts.filetype == 'Cord.new' then
    large_image = cfg.editor.icon
    large_text = config.get(cfg.editor.tooltip, opts)
  elseif cfg.display.swap_icons then
    large_image = cfg.editor.icon
    large_text = config.get(cfg.editor.tooltip, opts)
    small_image = opts.icon
    small_text = opts.tooltip or opts.filetype
  else
    large_image = opts.icon
    large_text = opts.tooltip or opts.filetype
    small_image = cfg.editor.icon
    small_text = config.get(cfg.editor.tooltip, opts)
  end

  print(large_image, large_text)

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

local function build_idle_activity(cfg, opts)
  local details, state
  if cfg.display.swap_fields then
    details = config.get(cfg.idle.state, opts)
    state = config.get(cfg.idle.details, opts)
  else
    details = config.get(cfg.idle.details, opts)
    state = config.get(cfg.idle.state, opts)
  end

  local large_image, large_text, small_image, small_text
  if cfg.display.swap_icons then
    large_image = cfg.editor.icon
    large_text = config.get(cfg.editor.tooltip, opts)
    small_image = config.get(cfg.idle.icon, opts)
    small_text = config.get(cfg.idle.tooltip, opts)
  else
    large_image = config.get(cfg.idle.icon, opts)
    large_text = config.get(cfg.idle.tooltip, opts)
    small_image = cfg.editor.icon
    small_text = config.get(cfg.editor.tooltip, opts)
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
    is_idle = cfg.idle.smart_idle,
  }
end

return {
  build_activity = build_activity,
  build_idle_activity = build_idle_activity,
}
