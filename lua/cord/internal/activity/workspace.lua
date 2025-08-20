local Future = require 'cord.core.async.future'
local async = require 'cord.core.async'
local fs = require 'cord.core.uv.fs'
local config = require 'cord.internal.config'
local logger = require 'cord.internal.log'

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
  logger.trace(function() return 'check_vcs_marker: checking ' .. marker_path end)
  local stat = fs.stat(marker_path):get()
  if not stat then return end
  logger.trace(
    function() return 'check_vcs_marker: found marker ' .. marker .. ' in ' .. curr_dir end
  )
  return curr_dir
end)

M.find_vcs_root = async.wrap(function(initial_path)
  logger.debug(function() return 'find_vcs_root: initial_path=' .. tostring(initial_path) end)
  if config.advanced.workspace.root_markers == nil then return end

  local curr_dir = initial_path
  local limit_to_cwd, cwd = config.advanced.workspace.limit_to_cwd, nil
  if limit_to_cwd then cwd = vim.fn.getcwd() end
  logger.trace(
    function()
      return 'find_vcs_root: limit_to_cwd=' .. tostring(limit_to_cwd) .. ', cwd=' .. tostring(cwd)
    end
  )

  while true do
    local marker_futures = {}
    for _, marker in ipairs(config.advanced.workspace.root_markers) do
      table.insert(marker_futures, check_vcs_marker(curr_dir, marker))
    end

    local results = Future.all(marker_futures):get()
    if not results then return end

    for _, result in ipairs(results) do
      if result then
        logger.debug(function() return 'find_vcs_root: found VCS root at ' .. result end)
        return result
      end
    end

    if limit_to_cwd and curr_dir == cwd then
      logger.trace(function() return 'find_vcs_root: reached CWD limit at ' .. curr_dir end)
      return curr_dir
    end

    local parent = vim.fn.fnamemodify(curr_dir, ':h')
    if parent == curr_dir then
      logger.trace(
        function() return 'find_vcs_root: reached filesystem root, returning initial_path' end
      )
      return initial_path
    end
    curr_dir = parent
  end
end)

M.find = async.wrap(function(initial_path)
  logger.debug(function() return 'find: initial_path=' .. tostring(initial_path) end)
  local has_initial_path = initial_path and initial_path ~= ''
  if not has_initial_path then initial_path = vim.fn.expand '%:p:h' end

  local protocol, path = initial_path:match '^(%w+)://+(.+)$'
  if protocol and M.protocol_handlers[protocol] then
    logger.trace(
      function() return 'find: protocol=' .. tostring(protocol) .. ', path=' .. tostring(path) end
    )
    local extracted_path, is_final = M.protocol_handlers[protocol](path)
    if not extracted_path or extracted_path == '' or extracted_path == '.' then
      logger.trace 'find: protocol handler returned invalid path, using CWD'
      return M.find_vcs_root(vim.fn.getcwd()):get()
    end

    if is_final then
      logger.trace(
        function() return 'find: protocol handler returned final path: ' .. extracted_path end
      )
      return extracted_path
    else
      logger.trace(
        function() return 'find: protocol handler returned path for VCS search: ' .. extracted_path end
      )
      return M.find_vcs_root(extracted_path):get()
    end
  end

  if has_initial_path then initial_path = vim.fn.fnamemodify(initial_path, ':h') end
  logger.trace(function() return 'find: searching VCS root from ' .. initial_path end)
  return M.find_vcs_root(initial_path):get()
end)

local function format_url(url)
  logger.trace(function() return 'format_url: input=' .. tostring(url) end)
  if url:find '^http' then
    local result = url:gsub('%.git$', '')
    logger.trace(function() return 'format_url: http url result=' .. result end)
    return result
  end

  local _, repo_url = url:match '^(.-)@(.+)$'
  if repo_url then
    repo_url = repo_url:gsub(':', '/', 1)
    local result = 'https://' .. repo_url:gsub('%.git$', '')
    logger.trace(function() return 'format_url: ssh url result=' .. result end)
    return result
  end

  logger.trace 'format_url: could not parse URL format'
  return nil
end

M.find_git_repository = async.wrap(function(workspace_path)
  logger.debug(
    function() return 'find_git_repository: workspace_path=' .. tostring(workspace_path) end
  )
  local config_path = workspace_path .. '/.git/config'

  local content = fs.readfile(config_path):get()
  if not content then
    logger.trace(
      function() return 'find_git_repository: no .git/config found at ' .. config_path end
    )
    return
  end

  local origin_url = content:match '%[remote "origin"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if origin_url then
    local formatted = format_url(vim.trim(origin_url))
    logger.debug(
      function() return 'find_git_repository: found origin URL: ' .. tostring(formatted) end
    )
    return formatted
  end

  local first_url = content:match '%[remote "[^"]+"%]%s*\n%s*url%s*=%s*([^\n]+)'
  if first_url then
    local formatted = format_url(vim.trim(first_url))
    logger.debug(
      function() return 'find_git_repository: found first remote URL: ' .. tostring(formatted) end
    )
    return formatted
  end

  logger.trace 'find_git_repository: no remote URLs found in .git/config'
  return
end)

return M
