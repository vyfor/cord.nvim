local uv = vim.loop or vim.uv

local VCS_MARKERS = {
  '.git',
  '.svn',
  '.hg',
}

local M = {}

M.find = function(initial_path)
  initial_path = initial_path:gsub('^%w+://+', '')
  initial_path = vim.fn.fnamemodify(initial_path, ':p:h')
  local curr_dir = initial_path

  while curr_dir and curr_dir ~= '' do
    for _, marker in ipairs(VCS_MARKERS) do
      local marker_path = curr_dir .. '/' .. marker
      local stat = uv.fs_stat(marker_path)
      if stat and stat.type == 'directory' then return curr_dir end
    end

    local parent = vim.fn.fnamemodify(curr_dir, ':h')
    if parent == curr_dir then break end
    curr_dir = parent
  end

  return initial_path
end

local function format_url(url)
  if url:find '^http' then return url:gsub('%.git$', '') end

  local _, repo_url = url:match '^(.-)@(.+)$'
  if repo_url then
    repo_url = repo_url:gsub(':', '/', 1)
    return 'https://' .. repo_url:gsub('%.git$', '')
  end

  return nil
end

M.find_git_repository = function(workspace_path)
  local config_path = workspace_path .. '/.git/config'

  local file = io.open(config_path, 'r')
  if not file then return nil end

  local content = file:read '*a'
  file:close()

  local origin_url =
    content:match '%[remote "origin"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if origin_url then return format_url(vim.trim(origin_url)) end

  local first_url = content:match '%[remote "[^"]+"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if first_url then return format_url(vim.trim(first_url)) end

  return nil
end

return M
