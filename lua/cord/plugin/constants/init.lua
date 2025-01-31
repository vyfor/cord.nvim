local M = {}

M.get_os = function()
  if M.os then return M.os end

  local uv = vim.loop or vim.uv
  local os_name = uv.os_uname().sysname:lower()
  local os_arch = uv.os_uname().machine
  if os_name:match 'windows' then
    os_name = 'windows'
  elseif os_name:match 'bsd$' then
    os_name = 'bsd'
  end
  if os_arch == 'arm64' then os_arch = 'aarch64' end
  if os_arch == 'i386' then os_arch = 'i686' end

  M.os = {
    name = os_name,
    arch = os_arch,
  }

  return M.os
end

M.get_pipe_path = function()
  if M.pipe_path then return M.pipe_path end

  M.pipe_path = M.get_os().name == 'windows' and '\\\\.\\pipe\\cord-ipc' or '/tmp/cord-ipc'

  return M.pipe_path
end

M.CLIENT_IDS = {
  vim = {
    id = '1219918645770059796',
    icon = 'vim',
  },
  neovim = {
    id = '1219918880005165137',
    icon = 'neovim',
  },
  lunarvim = {
    id = '1220295374087000104',
    icon = 'lunarvim',
  },
  nvchad = {
    id = '1220296082861326378',
    icon = 'nvchad',
  },
  astronvim = {
    id = '1230866983977746532',
    icon = 'astronvim',
  },
  lazyvim = {
    id = '1328074831601729567',
    icon = 'lazyvim',
  },
}

return M
