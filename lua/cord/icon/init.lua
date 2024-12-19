local M = {}

--- Increment this only when an existing icon is modified
---
--- Appended to the end of the asset URL for refetching; otherwise, it will be loaded from the cache, thus not being updated
M.ICONS_VERSION = '3'
M.ICONS_URL = 'https://raw.githubusercontent.com/vyfor/icons/master/icons/'
M.ICON_STYLE = 'onyx'

M.get = function(name, style)
  return M.ICONS_URL
    .. (style or M.ICON_STYLE)
    .. '/'
    .. name
    .. '.png?v='
    .. M.ICONS_VERSION
end

M.set_style = function(style) M.ICON_STYLE = style end

return M
