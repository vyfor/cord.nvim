local uv = vim.loop or vim.uv

local VCS_MARKERS = {
  '.git',
  '.svn',
  '.hg',
}

local M = {}

M.find = function(initial_path, callback)
  if not initial_path or initial_path == '' then return callback(nil) end

  initial_path = initial_path:gsub('^%w+://+', '')
  local curr_dir = initial_path

  local function check_marker(curr_dir)
    for _, marker in ipairs(VCS_MARKERS) do
      local marker_path = curr_dir .. '/' .. marker
      uv.fs_stat(marker_path, function(err, stat)
        if err then return callback(nil) end

        if stat and stat.type == 'directory' then
          callback(curr_dir)
        else
          local parent = vim.fn.fnamemodify(curr_dir, ':h')
          if parent == curr_dir then
            callback(initial_path)
          else
            check_marker(parent)
          end
        end
      end)
    end
  end

  check_marker(curr_dir)
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

M.find_git_repository = function(workspace_path, callback)
  local config_path = workspace_path .. '/.git/config'

  uv.fs_open(config_path, 'r', 438, function(err, fd)
    if err then return callback(nil) end

    uv.fs_read(fd, 4096, 0, function(err, content)
      uv.fs_close(fd)
      if err then return callback(nil) end

      local origin_url =
        content:match '%[remote "origin"%]%s*\n%s*url%s*=%s*([^\n]+)'
      if origin_url then
        callback(format_url(vim.trim(origin_url)))
        return
      end

      local first_url =
        content:match '%[remote "[^"]+"%]%s*\n%s*url%s*=%s*([^\n]+)'
      if first_url then
        callback(format_url(vim.trim(first_url)))
        return
      end

      callback(nil)
    end)
  end)
end

return M
