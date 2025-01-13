local Future = require 'cord.core.async.future'
local async = require 'cord.core.async'
local fs = require 'cord.core.uv.fs'

local VCS_MARKERS = {
  '.git',
  '.svn',
  '.hg',
}

local M = {}

local check_vcs_marker = async.wrap(function(curr_dir, marker)
  local marker_path = curr_dir .. '/' .. marker
  local stat = fs.stat(marker_path):get()
  if not stat then return end
  return (stat.type == 'directory' and curr_dir)
end)

M.find = async.wrap(function(initial_path)
  if not initial_path or initial_path == '' then return end

  initial_path = initial_path:gsub('^%w+://+', '')
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

  local origin_url =
    content:match '%[remote "origin"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if origin_url then return format_url(vim.trim(origin_url)) end

  local first_url = content:match '%[remote "[^"]+"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if first_url then return format_url(vim.trim(first_url)) end
end)

return M
