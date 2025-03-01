local Future = require 'cord.core.async.future'
local async = require 'cord.core.async'
local fs = require 'cord.core.uv.fs'

local VCS_MARKERS = {
  '.git',
  '.svn',
  '.hg',
}

local M = {}

M.protocol_handlers = {
  term = function(path)
    local cwd = path:match '^(.-)//%d+:'
    if cwd then return vim.fn.expand(cwd), false end
  end,
  man = function(path) return path, true end,
}

local check_vcs_marker = async.wrap(function(curr_dir, marker)
  local marker_path = curr_dir .. '/' .. marker
  local stat = fs.stat(marker_path):get()
  if not stat then return end
  return (stat.type == 'directory' and curr_dir)
end)

M.find_vcs_root = async.wrap(function(initial_path)
  local curr_dir = initial_path

  while true do
    local marker_futures = {}
    for _, marker in ipairs(VCS_MARKERS) do
      table.insert(marker_futures, check_vcs_marker(curr_dir, marker))
    end

    local results = Future.all(marker_futures):get()
    if not results then return end

    for _, result in ipairs(results) do
      if result then return result end
    end

    local parent = vim.fn.fnamemodify(curr_dir, ':h')
    if parent == curr_dir then return initial_path end
    curr_dir = parent
  end
end)

M.find = async.wrap(function(initial_path)
  if not initial_path or initial_path == '' then return end

  local protocol, path = initial_path:match '^(%w+)://+(.+)$'
  if protocol and M.protocol_handlers[protocol] then
    local extracted_path, is_final = M.protocol_handlers[protocol](path)
    if not extracted_path or extracted_path == '' or extracted_path == '.' then
      return M.find_vcs_root(vim.fn.getcwd()):get()
    end

    if is_final then
      return extracted_path
    else
      return M.find_vcs_root(extracted_path):get()
    end
  end

  initial_path = vim.fn.fnamemodify(initial_path, ':h')
  return M.find_vcs_root(initial_path):get()
end)

local function format_url(url)
  if url:find '^http' then return url:gsub('%.git$', '') end

  local _, repo_url = url:match '^(.-)@(.+)$'
  if repo_url then
    repo_url = repo_url:gsub(':', '/', 1)
    return 'https://' .. repo_url:gsub('%.git$', '')
  end

  return nil
end

M.find_git_repository = async.wrap(function(workspace_path)
  local config_path = workspace_path .. '/.git/config'

  local content = fs.readfile(config_path):get()
  if not content then return end

  local origin_url = content:match '%[remote "origin"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if origin_url then return format_url(vim.trim(origin_url)) end

  local first_url = content:match '%[remote "[^"]+"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if first_url then return format_url(vim.trim(first_url)) end
end)

return M
