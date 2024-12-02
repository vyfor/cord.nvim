local logger = require 'cord.util.logger'
local constants = require 'cord.util.constants'

local os_name = vim.loop.os_uname().sysname
if os_name:find('Windows', 1, true) == 1 then
  os_name = 'Windows'
elseif os_name:match 'BSD$' then
  os_name = 'BSD'
end

local function file_exists(filename)
  local stat = vim.loop.fs_stat(filename)
  return stat and stat.type == 'file'
end

local function move_file(src, dest)
  local result, err = os.rename(src, dest)
  if not result then logger.error('Error moving file: ' .. err) end
end

local function get_icon(config, filename, filetype)
  if not config.assets then return end

  local icon = config.assets[filetype]
  if icon then return icon end

  icon = config.assets[filename]
  if icon then return icon end

  local extension = filename:match '(%.[^%.]+)$'
  icon = config.assets[extension]
  if icon then return icon end

  icon = config.assets['Cord.override']
  if icon then return icon, 'Cord.override' end
end

local function get_asset(type, name)
  return constants.ASSETS_URL
    .. '/'
    .. type
    .. '/'
    .. name
    .. '.png?v='
    .. constants.ASSETS_VERSION
end

return {
  os_name = os_name,
  file_exists = file_exists,
  move_file = move_file,
  get_icon = get_icon,
  get_asset = get_asset,
}
