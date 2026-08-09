local async = require 'cord.core.async'
local logger = require 'cord.api.log'

local M = {}

local function get_os_info()
  local uv = vim.loop or vim.uv
  local uname = uv.os_uname()
  return {
    sysname = uname.sysname,
    release = uname.release,
    version = uname.version,
    machine = uname.machine,
    wsl = os.getenv 'WSL_DISTRO_NAME',
  }
end

local function get_git_sha()
  local root = require('cord.server.fs').get_plugin_root()
  local fs = require 'cord.core.uv.fs'
  local git_dir = fs.stat(root .. '/.git'):await()
  if not git_dir then return 'unknown' end

  if not vim.fn.executable 'git' then return 'git not found' end

  local process = require 'cord.core.uv.process'
  local res = process
    .spawn({
      cmd = 'git',
      args = { 'rev-parse', '--short', 'HEAD' },
      cwd = root,
    })
    :await()

  if not res or res.code ~= 0 then return 'unknown' end
  local sha = res.stdout:gsub('^%s*(.-)%s*$', '%1')
  if sha == '' then return 'unknown' end
  return sha
end

local function get_server_info()
  local fs = require 'cord.core.uv.fs'
  local config = require('cord.api.config').get()
  local exec_path = require('cord.server.fs').get_executable_path(config)
  local stat = fs.stat(exec_path):await()
  if not stat then return { version = 'not installed', path = exec_path, executable = false } end

  local is_windows = get_os_info().sysname:lower():match 'windows'
  local is_exec = true
  if not is_windows then is_exec = vim.fn.executable(exec_path) == 1 end

  local version = 'unknown'
  if is_exec then
    local process = require 'cord.core.uv.process'
    local res = process
      .spawn({
        cmd = exec_path,
        args = { '-v' },
      })
      :await()

    if res and res.code == 0 then
      version = res.stdout:gsub('^%s*(.-)%s*$', '%1')
      if version == '' then version = 'unknown' end
    end
  end

  return { version = version, path = exec_path, executable = is_exec }
end

local function get_server_metadata()
  local fs = require 'cord.core.uv.fs'
  local path = require('cord.server.fs').get_plugin_root() .. '/.github/server-metadata.txt'
  local content = fs.readfile(path):await()
  if not content then return nil end
  local trimmed = content:gsub('^%s*(.-)%s*$', '%1')
  if trimmed == '' then return nil end
  return trimmed
end

local function get_status()
  local cord = require 'cord.server'
  local status_map = {
    disconnected = 'disconnected',
    initializing = 'initializing',
    initialized = 'connected to server',
    connecting = 'connecting to discord',
    connected = 'handshaking with discord',
    ready = 'connected to discord',
  }
  local status = status_map[cord.status] or 'unknown'

  local pipe_path = require('cord.api.config').get().advanced.server.pipe_path
    or require('cord.core.util').get_pipe_path()

  local fs = require 'cord.core.uv.fs'
  local pipe_exists = false
  local stat = fs.stat(pipe_path):await()
  if stat then pipe_exists = true end

  local client_alive = cord.client and not cord.client:is_closing()
  local manager_alive = cord.manager ~= nil

  return {
    status = status,
    pipe_path = pipe_path,
    pipe_exists = pipe_exists,
    client_alive = client_alive,
    manager_alive = manager_alive,
  }
end

local function probe_pipe(path)
  local pipe = require('cord.core.uv.pipe').new()
  local _, err = pipe:connect(path):await()
  pipe:close()
  return err == nil
end

local function check_discord_pipes()
  local config = require('cord.api.config').get()
  local custom = config.advanced.discord.pipe_paths
  local is_windows = get_os_info().sysname:lower():match 'windows'

  local function get_paths()
    if custom and #custom > 0 then return custom end

    local paths = {}
    if is_windows then
      for i = 0, 10 do
        paths[#paths + 1] = '\\\\.\\pipe\\discord-ipc-' .. i
      end
      return paths
    end

    local uv = vim.loop or vim.uv
    local bases = {
      os.getenv 'XDG_RUNTIME_DIR',
      os.getenv 'TMPDIR',
      os.getenv 'TMP',
      os.getenv 'TEMP',
      '/tmp',
    }

    local seen = {}
    for _, base in ipairs(bases) do
      if base and base ~= '' and not seen[base] then
        seen[base] = true
        for i = 0, 10 do
          local path = base .. '/discord-ipc-' .. i
          local stat = uv.fs_stat(path)
          if stat and stat.type == 'socket' then paths[#paths + 1] = path end
        end
      end
    end

    return paths
  end

  local paths = get_paths()
  if #paths == 0 then return {} end

  local results = {}
  for _, path in ipairs(paths) do
    local ok = probe_pipe(path)
    results[#results + 1] = { path = path, reachable = ok }
  end
  return results
end

local function build_report()
  local lines = {}

  local function section(title)
    table.insert(lines, '')
    table.insert(lines, '### ' .. title)
    table.insert(lines, '')
  end

  local function kv(key, val) table.insert(lines, '**`' .. key .. '`**: `' .. tostring(val) .. '`') end

  local function code_block(lang, text)
    table.insert(lines, '```' .. lang)
    for _, line in ipairs(vim.split(text, '\n')) do
      table.insert(lines, line)
    end
    table.insert(lines, '```')
  end

  section 'version'
  local server = get_server_info()
  kv('server', server.version)
  kv('executable', server.executable)
  kv('executable path', server.path)
  kv('git sha', get_git_sha())
  kv('server metadata', get_server_metadata() or 'not found')
  kv('neovim', tostring(vim.version()))
  kv('lua', _VERSION .. (jit and ' (luajit)' or ''))

  if not server.executable then
    section 'warnings'
    table.insert(lines, '- server is not executable')
  else
    section 'warnings'
    local has_warnings = false
    if vim.fn.executable 'curl' ~= 1 then
      table.insert(lines, '- curl not found')
      has_warnings = true
    end
    if not has_warnings then table.insert(lines, 'none') end
  end

  section 'status'
  local status = get_status()
  kv('status', status.status)
  kv('pipe path', status.pipe_path)
  kv('pipe exists', status.pipe_exists)
  kv('client alive', status.client_alive)
  kv('manager alive', status.manager_alive)

  section 'discord rpc'
  local custom = require('cord.api.config').get().advanced.discord.pipe_paths
  if custom and #custom > 0 then table.insert(lines, '(using custom pipe paths)') end
  local pipes = check_discord_pipes()
  if #pipes == 0 then
    table.insert(lines, 'none found')
  else
    for _, p in ipairs(pipes) do
      table.insert(
        lines,
        '- `' .. p.path .. '`: ' .. (p.reachable and 'reachable' or 'unreachable')
      )
    end
  end

  section 'os'
  local os_info = get_os_info()
  kv('sysname', os_info.sysname)
  kv('release', os_info.release)
  kv('machine', os_info.machine)
  if os_info.wsl then kv('wsl', os_info.wsl) end

  section 'config validation'
  local results = require('cord.api.config').validate(require('cord').user_config)
  if results.is_valid then
    table.insert(lines, 'no issues found')
  else
    for _, err in ipairs(results.errors) do
      table.insert(lines, '- error: ' .. err.msg)
      if err.hint then table.insert(lines, '  hint: ' .. err.hint) end
    end
    for _, warn in ipairs(results.warnings) do
      table.insert(lines, '- warning: ' .. warn)
    end
  end

  section 'config (user)'
  local user_config = require('cord').user_config or {}
  if next(user_config) == nil then
    table.insert(lines, '(none)')
  else
    code_block('lua', vim.inspect(user_config))
  end

  section 'config (merged)'
  local merged = require('cord.api.config').get()
  code_block('lua', vim.inspect(merged))

  return table.concat(lines, '\n')
end

M.report = async.wrap(function() return build_report() end)

M.show = function()
  async.run(function()
    local report = M.report():unwrap()
    vim.schedule(function()
      local buf = vim.api.nvim_create_buf(false, true)
      vim.api.nvim_buf_set_lines(buf, 0, -1, false, vim.split(report, '\n'))
      vim.bo[buf].filetype = 'markdown'
      vim.api.nvim_win_set_buf(0, buf)
      vim.api.nvim_buf_set_name(buf, 'cord_debug')
    end)
  end)
end

M.copy = function()
  async.run(function()
    local report = M.report():unwrap()
    vim.schedule(function()
      vim.fn.setreg('+', report)
      logger.notify('copied to clipboard', vim.log.levels.INFO)
    end)
  end)
end

return M
