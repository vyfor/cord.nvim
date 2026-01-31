local logger = require 'cord.api.log'
local config = require 'cord.api.config'
local mappings = require 'cord.internal.activity.mappings'
local async = require 'cord.core.async'
local fs = require 'cord.core.uv.fs'
local ws_utils = require 'cord.internal.activity.workspace'

local M = {}

M.name = 'resolver'
M.description = 'Collection of resolvers for dynamic filetype detection'

M.config = {
  sources = {
    nestjs = false,
    toggleterm = false,
    oil = false,
  },
}

local cache = {
  nestjs = {},
}

local has_file = async.wrap(function(dir, filename)
  local path = dir .. '/' .. filename
  local stat = fs.stat(path):await()
  return stat ~= nil
end)

local sources = {
  nestjs = {
    event = { 'pre_activity' },
    match = {
      filetype = {
        'angular',
        'javascript',
        'typescript',
      },
    },
    run = async.wrap(function(opts)
      local ws = opts.workspace_dir or vim.uv.cwd()
      local cached = cache.nestjs[ws]

      if cached == nil then
        cached = has_file(ws, 'nest-cli.json'):await()
        cache.nestjs[ws] = cached
      end

      if cached then opts.force_filetype = 'nest' end
    end),
  },
  toggleterm = {
    event = { 'pre_activity' },
    match = { filetype = 'toggleterm' },
    run = function(opts)
      local cmd = opts.filename:match '^([^%s;]+)'
      if cmd and cmd ~= '' then
        local asset = config.assets[cmd]
        local fallback = mappings.filetype_mappings[cmd]
        local name
        if asset then
          name = asset.tooltip
          if type(name) == 'function' then name = name(opts) end

          if name == nil then name = fallback and fallback[3] or cmd end
        elseif fallback then
          name = fallback[3]
        else
          name = cmd
        end

        opts.filename = name
        opts.force_filetype = cmd
      end
    end,
  },
  oil = {
    event = { 'workspace_change' },
    match = {
      workspace = '.',
      filetype = { 'oil', 'oil_preview', 'oil_progress' },
    },
    run = async.wrap(function(opts)
      local path = vim.fn.expand '%:p:h'
      if path:sub(1, 7) == 'oil:///' then
        local stripped = path:sub(7)
        local cached = opts.manager.workspace:get(stripped)

        local repo_url = cached and cached.repo_url or nil
        if not repo_url then
          repo_url = ws_utils.find_git_repository(stripped):await()
        end

        local info = {
          dir = stripped,
          name = vim.fn.fnamemodify(stripped, ':t'),
          repo_url = repo_url,
        }

        opts.manager.workspace:set(path, info)
        opts.manager.workspace:set_current(info)

        opts.workspace = info.name
        opts.workspace_dir = info.dir
        opts.repo_url = info.repo_url
      end
    end),
  },
}

local active_resolvers = {}

local function check_match(match, opts)
  if not match then return true end

  if type(match) == 'function' then return match(opts) end

  for k, v in pairs(match) do
    if type(v) == 'table' then
      if not vim.tbl_contains(v, opts[k]) then return false end
    elseif opts[k] ~= v then
      return false
    end
  end

  return true
end

function M.setup(user_config)
  if user_config then M.config = vim.tbl_deep_extend('force', M.config, user_config) end

  local user_resolvers = user_config and user_config.resolvers
  if type(user_resolvers) == 'boolean' then
    user_resolvers = { user_resolvers }
  elseif type(user_resolvers) ~= 'table' then
    user_resolvers = {}
  end
  local global_toggle = user_resolvers[1]

  for name, source in pairs(sources) do
    local enabled = user_resolvers[name]
    if enabled == nil then
      if global_toggle ~= nil then
        enabled = global_toggle
      else
        enabled = M.config.sources[name]
      end
    end

    if enabled then
      for _, event in ipairs(source.event) do
        if not active_resolvers[event] then active_resolvers[event] = {} end
        table.insert(active_resolvers[event], {
          name = name,
          source = source,
          is_async = async.is_async(source.run),
        })
      end

      logger.debug('Resolver: Registered resolver ' .. name)
    end
  end

  local priorities = require('cord.internal.hooks').PRIORITY
  M.hooks = {}
  for event, resolvers in pairs(active_resolvers) do
    M.hooks[event] = {
      fun = async.wrap(function(opts)
        for _, resolver in ipairs(resolvers) do
          if check_match(resolver.source.match, opts) then
            logger.debug('Resolver: Running resolver ' .. resolver.name)
            local result = resolver.source.run(opts)
            if resolver.is_async then result:await() end
          end
        end
      end),
      priority = priorities.HIGHEST,
    }
  end

  return M
end

return M
