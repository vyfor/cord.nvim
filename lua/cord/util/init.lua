local constants = require 'cord.util.constants'
local logger = require 'cord.util.logger'

local uv = vim.loop or vim.uv

local path_sep = '/'
local os_name = uv.os_uname().sysname
if os_name:find('Windows', 1, true) == 1 then
  path_sep = '\\'
  os_name = 'Windows'
elseif os_name:match 'BSD$' then
  os_name = 'BSD'
end

local function move_file(src, dest, callback)
  uv.fs_copyfile(src, dest, { ficlone = true }, function(copy_err)
    if copy_err then
      callback(nil, 'Failed to copy file: ' .. copy_err)
      return
    end
    uv.fs_unlink(src, function(del_err)
      if del_err then
        logger.warn('Could not remove source file after copy: ' .. del_err)
      end
      callback()
    end)
  end)
end

local function rm_file(filename, callback) uv.fs_unlink(filename, callback) end

local function mkdir(path, callback) uv.fs_mkdir(path, 493, callback) end

local function kill_process(pid) uv.kill(pid, 15) end

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
  path_sep = path_sep,
  os_name = os_name,
  move_file = move_file,
  rm_file = rm_file,
  mkdir = mkdir,
  kill_process = kill_process,
  get_icon = get_icon,
  get_asset = get_asset,
}
