local uv = vim.loop or vim.uv
local logger = require 'cord.api.log'

local M = {
  config = {
    override = true,
    precedence = 'blacklist',
    rules = {
      blacklist = {},
      whitelist = {},
    },
    resolve_symlinks = true,
    action = nil,
    fallback = nil,
  },
  cache = {},
  pending = nil,
  manager = nil,
  workspace = nil,
  workspace_dir = nil,
}

local function cache_key(workspace)
  local path = vim.api.nvim_buf_get_name(0)
  return tostring(workspace) .. '::' .. path
end

local function normalize_path(path)
  if not path then return nil end
  local norm = vim.fs.normalize(path)
  if M.config.resolve_symlinks then
    local ok, real = pcall(uv.fs_realpath, norm)
    if ok and real then return real end
  end
  return norm
end

local function match_path(rule, file_dir)
  if not rule then return false end
  if not file_dir or file_dir == '' then return false end
  local dir = normalize_path(file_dir)
  return rule == dir
end

local function match_name(rule, workspace_name)
  if not (rule and workspace_name) then return false end
  return rule == workspace_name
end

local function match_glob(rule, file_dir)
  if not rule then return false end
  if not file_dir or file_dir == '' then return false end
  local dir = normalize_path(file_dir)
  return vim.fn.match(tostring(dir), rule) ~= -1
end

local function rule_matches(rule, bufname)
  if type(rule) == 'function' then
    local ok, res = pcall(rule, {
      rule = rule,
      workspace = M.workspace,
    })
    return ok and res and res ~= false
  elseif type(rule) == 'table' then
    local ty = rule.type
    local val = rule.value
    if type(val) == 'function' then
      local ok, res = pcall(val, {
        rule = rule,
        workspace = M.workspace,
        workspace_dir = M.workspace_dir,
      })
      return ok and res and res ~= false
    end
    if ty == 'path' then
      return match_path(val, M.workspace_dir)
    elseif ty == 'name' then
      return match_name(val, M.workspace)
    elseif ty == 'glob' then
      return match_glob(val, bufname)
    end
  end
  return false
end

local function find_matching_rule(list, bufname)
  for i = 1, #list do
    local r = list[i]
    if rule_matches(r, bufname) then return r end
  end
  return nil
end

local function is_visible()
  local bufname = vim.api.nvim_buf_get_name(0)

  local cached = M.cache[cache_key(M.workspace)]
  if cached then return cached.visible, cached.rule end

  local prec = M.config.precedence
  local rules = M.config.rules or {}

  local whitelist = rules.whitelist or {}
  local blacklist = rules.blacklist or {}

  local has_whitelist = #whitelist > 0
  local has_blacklist = #blacklist > 0

  local matched_whitelist = find_matching_rule(whitelist, bufname)
  local matched_blacklist = find_matching_rule(blacklist, bufname)

  local visible
  local rule

  if matched_whitelist and matched_blacklist then
    local is_wl = prec == 'whitelist'
    visible = is_wl
    rule = is_wl and matched_whitelist or matched_blacklist
  elseif has_whitelist then
    visible = matched_whitelist ~= nil
    rule = matched_whitelist
  elseif has_blacklist then
    visible = matched_blacklist == nil
    rule = matched_blacklist
  else
    visible = true
    rule = nil
  end

  M.cache[cache_key(M.workspace)] = { visible = visible, rule = rule }
  return visible, rule
end

M.check_visibility = function(pending)
  if not pending then
    M.pending = true
    return
  end

  local visible, rule = is_visible()
  if not rule then
    if M.config.fallback then
      local ok, res = pcall(M.config.fallback, {
        visible = visible,
        rule = rule,
        workspace = M.workspace,
        workspace_dir = M.workspace_dir,
      })
      if not ok then
        logger.notify(
          function() return ('fallback execution failed for config: %s'):format(res) end,
          vim.log.levels.ERROR
        )
      end
      return
    end
  end

  if rule and rule.action then
    local ok, res = pcall(rule.action, {
      visible = visible,
      rule = rule,
      workspace = M.workspace,
      workspace_dir = M.workspace_dir,
    })
    if not ok then
      logger.notify(
        function() return ('action execution failed for rule %s: %s'):format(vim.inspect(rule), res) end,
        vim.log.levels.ERROR
      )
    end
    return
  elseif M.config.action then
    local ok, res = pcall(M.config.action, {
      visible = visible,
      rule = rule,
      workspace = M.workspace,
      workspace_dir = M.workspace_dir,
    })
    if not ok then
      logger.notify(
        function() return ('action execution failed for config: %s'):format(res) end,
        vim.log.levels.ERROR
      )
    end
    return
  end

  if visible then
    logger.debug(
      function()
        return 'Visibility: resuming activities'
          .. (rule and (' due to rule: ' .. vim.inspect(rule)) or '')
      end
    )
    if M.manager then M.manager:resume() end
  else
    logger.debug(
      function()
        return 'Visibility: suppressing activities'
          .. (rule and (' due to rule: ' .. vim.inspect(rule)) or '')
      end
    )
    if M.manager then M.manager:suppress() end
  end
end

M.validate = function(config)
  if config.precedence ~= 'whitelist' and config.precedence ~= 'blacklist' then
    return 'invalid precedence; must be "whitelist" or "blacklist"'
  end
  if type(config.rules) ~= 'table' then return 'rules must be a table' end

  local function is_pathlike(s) return s:find '[/\\]' ~= nil end

  local function normalize_rule(r)
    if type(r) == 'string' then
      if is_pathlike(r) then
        return {
          type = 'path',
          value = normalize_path(r),
        }
      else
        return {
          type = 'name',
          value = r,
        }
      end
    elseif type(r) == 'table' then
      local ty = r.type
      local val = r.value
      local action = r.action

      if action and type(action) ~= 'function' then return nil, 'actions must be functions' end

      if type(val) == 'function' and ty ~= nil then
        return nil, 'function values do not support type field'
      end

      if ty == 'path' and type(val) == 'string' then
        r.value = normalize_path(val)
      elseif ty == 'glob' and type(val) == 'string' then
        local ok, result = pcall(vim.fn.glob2regpat, val)
        if ok then
          r.value = result
        else
          return nil, 'Invalid glob pattern: ' .. tostring(val) .. ' (' .. tostring(result) .. ')'
        end
      end
      return r
    elseif type(r) == 'function' then
      return r
    else
      return nil, 'rules must be strings, tables, or functions'
    end
  end

  local rules = config.rules
  for _, listname in ipairs { 'whitelist', 'blacklist' } do
    local lst = rules[listname]
    if type(lst) == 'table' then
      for i = 1, #lst do
        local norm, err = normalize_rule(lst[i])
        if err then return err end
        lst[i] = norm
      end
    end
  end
end

M.setup = function(config)
  if config then
    config = vim.tbl_deep_extend('force', M.config, config)
    local err = M.validate(config)
    if err then error(err, 0) end
    M.config = config
  end

  local initialized = false

  return {
    name = 'Visibility',
    description = 'Control visibility of workspace based on rules',
    variables = {
      has_match = function()
        local visible, rule = is_visible()
        return { visible = visible, rule = rule }
      end,
    },
    hooks = M.config.override and {
      ready = function(m) M.manager = m end,
      workspace_change = function(opts)
        M.workspace = normalize_path(opts.workspace)
        M.workspace_dir = normalize_path(opts.workspace_dir)

        local pending = M.pending
        if pending then
          M.pending = nil
          M.check_visibility(true)
          return
        end

        if not initialized then
          initialized = true
          M.check_visibility()
        end
      end,
      buf_enter = function() M.check_visibility() end,
    } or nil,
  }
end

return M
