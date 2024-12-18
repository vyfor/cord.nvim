--- Increment this only when an existing icon is modified
---
--- Appended to the end of the asset URL for refetching; otherwise, it will be loaded from the cache, thus not being updated
local ICONS_VERSION = '2'
local ICONS_URL = 'https://raw.githubusercontent.com/vyfor/icons/master/icons/'
local DEFAULT_ICON_STYLE = 'onyx'

local function get(name, style)
  return ICONS_URL
    .. (style or DEFAULT_ICON_STYLE)
    .. '/'
    .. name
    .. '.png?v='
    .. ICONS_VERSION
end

local function set_style(style) DEFAULT_ICON_STYLE = style end

return {
  style = DEFAULT_ICON_STYLE,
  get = get,
  set_style = set_style,
}
