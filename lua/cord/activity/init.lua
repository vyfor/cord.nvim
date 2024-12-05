local config = require 'cord.util.config'
local mappings = require 'cord.mappings'
local utils = require 'cord.util'

local function build_activity(cfg, opts)
  if opts.filetype == '' then
    if opts.filename == '' then
      opts.filetype = 'Cord.new'
    else
      opts.filetype = 'Cord.unknown'
    end
  end

  local type, icon, tooltip = mappings.get(opts.filetype, opts.filename)
  opts.type = type
  opts.icon = utils.get_asset(type, icon)
  opts.tooltip = tooltip

  local custom_icon, override_type =
    utils.get_icon(cfg, opts.filename, opts.filetype)
  if custom_icon then
    opts.name = custom_icon.name
    opts.icon = custom_icon.icon or icon
    opts.tooltip = custom_icon.tooltip or tooltip
    opts.type = custom_icon.type or type
    opts.text = custom_icon.text
    opts.filetype = override_type or opts.filetype
  end

  local file_text
  if opts.text then
    file_text = opts.text
  else
    if type == 'language' then
      if opts.is_read_only then
        file_text = config.get(cfg.text.viewing, opts)
      else
        file_text = config.get(cfg.text.editing, opts)
      end
    elseif type == 'file_browser' then
      file_text = config.get(cfg.text.file_browser, opts)
    elseif type == 'plugin_manager' then
      file_text = config.get(cfg.text.plugin_manager, opts)
    elseif type == 'lsp' then
      file_text = config.get(cfg.text.lsp, opts)
    elseif type == 'vcs' then
      file_text = config.get(cfg.text.vcs, opts)
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
    small_text = opts.tooltip
  else
    large_image = opts.icon
    large_text = opts.tooltip
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
      or utils.get_asset('editor', 'idle')
    small_text = config.get(cfg.idle.tooltip, opts)
  else
    large_image = config.get(cfg.idle.icon, opts)
      or utils.get_asset('editor', 'idle')
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
