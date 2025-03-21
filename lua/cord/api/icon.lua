local M = {}

--- Increment this only when an existing icon is modified
---
--- Appended to the end of the asset URL for refetching; otherwise, it will be loaded from the cache, thus not being updated
M.ICONS_VERSION = '16'
M.ICONS_URL = 'https://raw.githubusercontent.com/vyfor/icons/master/icons/'
M.ICON_THEME = 'default'
M.THEME_FLAVOR = 'dark'

M.supported_themes = { 'default', 'atom', 'catppuccin' }
M.supported_flavors = { 'dark', 'light', 'accent' }

M.get = function(name, theme, flavor)
  return M.ICONS_URL
    .. (theme or M.ICON_THEME)
    .. '/'
    .. (flavor or M.THEME_FLAVOR)
    .. '/'
    .. name
    .. '.png?v='
    .. M.ICONS_VERSION
end

M.set = function(theme, flavor)
  if not vim.tbl_contains(M.supported_themes, theme) then
    require('cord.plugin.log').log_raw(vim.log.levels.WARN, 'Unknown theme: ' .. theme)
  end

  if not vim.tbl_contains(M.supported_flavors, flavor) then
    require('cord.plugin.log').log_raw(vim.log.levels.WARN, 'Unknown flavor: ' .. flavor)
  end

  M.ICON_THEME = theme
  M.THEME_FLAVOR = flavor
  M.DEFAULT_IDLE_ICON = (theme == 'default' and flavor ~= 'accent') and 'keyboard' or 'idle'
end

return M
