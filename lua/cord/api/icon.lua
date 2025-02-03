local M = {}

--- Increment this only when an existing icon is modified
---
--- Appended to the end of the asset URL for refetching; otherwise, it will be loaded from the cache, thus not being updated
M.ICONS_VERSION = '12'
M.ICONS_URL = 'https://raw.githubusercontent.com/vyfor/icons/master/icons/'
M.ICON_THEME = 'onyx'

M.get = function(name, theme)
  return M.ICONS_URL .. (theme or M.ICON_THEME) .. '/' .. name .. '.png?v=' .. M.ICONS_VERSION
end

M.set_theme = function(theme)
  M.ICON_THEME = theme
  M.DEFAULT_IDLE_ICON = (theme == 'onyx') and 'keyboard' or 'idle'
end

return M
