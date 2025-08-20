local M = {}

function M.url_encode(str)
  return (str:gsub('([^%w%-_%.~])', function(c) return string.format('%%%02X', string.byte(c)) end))
end

function M.tbl_flatten(t)
  local result = {}

  local function flatten(sub)
    for _, v in ipairs(sub) do
      if type(v) == 'table' then
        flatten(v)
      elseif v then
        table.insert(result, v)
      end
    end
  end

  flatten(t)
  return result
end

function M.get_os()
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

function M.get_pipe_path()
  if M.pipe_path then return M.pipe_path end

  M.pipe_path = M.get_os().name == 'windows' and '\\\\.\\pipe\\cord-ipc' or '/tmp/cord-ipc'

  return M.pipe_path
end

return M
