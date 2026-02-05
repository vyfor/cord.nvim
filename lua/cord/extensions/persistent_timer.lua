local M = {
  timers = {},
  current_key = nil,
  current_mode = nil,
  last_save_time = 0,
  instance_id = nil,
  session_start_time = nil,
  save_in_progress = false,
  is_initialized = false,
  config = {
    ---Defines the scope of time tracking.
    ---@type 'workspace' | 'file' | 'filetype' | 'global'
    scope = 'workspace',
    ---The absolute path to the file where timer data will be saved.
    ---@type string
    file = vim.fn.stdpath 'data' .. '/cord/extensions/persistent_timer/data.json',
    ---The primary mode of time tracking to use.
    ---@type 'active' | 'idle' | 'all'
    mode = 'all',
    ---A list of events that should trigger a save operation.
    ---@type ('exit' | 'focus_change' | 'periodic')[]
    save_on = { 'exit', 'focus_change', 'periodic' },
    ---The interval in seconds for the 'periodic' save trigger.
    ---@type integer
    save_interval = 30,
  },
}

local FS = require 'cord.core.uv.fs'
local Async = require 'cord.core.async'
local logger = require 'cord.api.log'
local uv = vim.loop or vim.uv

local function now() return os.time() end

local function get_dir(path) return path:match '^(.*)[/\\]' end

local function generate_instance_id()
  return vim.fn.getpid() .. '_' .. os.time() .. '_' .. math.random(1000, 9999)
end

local function get_key(opts)
  if M.config.scope == 'workspace' then
    return opts.workspace_dir or vim.fn.getcwd()
  elseif M.config.scope == 'file' then
    local p = vim.fn.expand '%:p'
    return (p and p ~= '') and p or 'CORD.UNKNOWN_FILE'
  elseif M.config.scope == 'filetype' then
    return opts.filetype or 'unknown'
  elseif M.config.scope == 'global' then
    return 'CORD.GLOBAL'
  end
  return nil
end

local function ensure_key(key)
  if not M.timers[key] then
    M.timers[key] = {
      modes = {
        active = { total_time = 0, session_start_time = nil, display_start_time = nil },
        idle = { total_time = 0, session_start_time = nil, display_start_time = nil },
        all = { total_time = 0, session_start_time = nil, display_start_time = nil },
      },
    }
  end
  return M.timers[key]
end

local function stop_timer(key)
  if not key or not M.timers[key] then return end
  local timer = M.timers[key]

  for _, bucket in pairs(timer.modes) do
    if bucket.session_start_time then
      local elapsed = now() - bucket.session_start_time
      if elapsed > 0 then bucket.total_time = bucket.total_time + elapsed end
      bucket.session_start_time = nil
      bucket.display_start_time = nil
    end
  end
end

local function start_timer(key, mode)
  if not key then return end
  local timer = ensure_key(key)
  M.current_key = key
  M.current_mode = mode

  local buckets_to_start = { 'all' }
  if mode ~= 'all' then table.insert(buckets_to_start, mode) end

  for _, bucket_name in ipairs(buckets_to_start) do
    local bucket = timer.modes[bucket_name]
    if bucket and not bucket.session_start_time then bucket.session_start_time = now() end
  end
end

local function merge_timers(other_data)
  for key, other in pairs(other_data) do
    local ours = ensure_key(key)
    if other.modes then
      for mode_name, other_bucket in pairs(other.modes) do
        local our_bucket = ours.modes[mode_name]
        if our_bucket and type(other_bucket.total_time) == 'number' then
          our_bucket.total_time = math.max(our_bucket.total_time, other_bucket.total_time)
        end
      end
    end
  end
end

local load_timers = Async.wrap(function()
  local content, err = FS.readfile(M.config.file):await()
  if err then
    logger.debug('Could not read timer file (may not exist yet): ' .. tostring(err))
    return
  end
  if not content or content == '' then return end

  local ok, data = pcall(vim.json.decode, content)
  if ok and type(data) == 'table' then
    merge_timers(data)
  else
    logger.debug 'Failed to decode JSON from timer file.'
  end
end)

local function get_snapshot()
  local timers_to_save = vim.deepcopy(M.timers)
  if M.current_key and timers_to_save[M.current_key] then
    local timer = timers_to_save[M.current_key]
    for _, bucket in pairs(timer.modes) do
      if bucket.session_start_time then
        local elapsed = now() - bucket.session_start_time
        if elapsed > 0 then bucket.total_time = bucket.total_time + elapsed end
      end
    end
  end

  for _, timer in pairs(timers_to_save) do
    for _, bucket in pairs(timer.modes) do
      bucket.session_start_time = nil
      bucket.display_start_time = nil
    end
  end

  return timers_to_save
end

local save_timers = Async.wrap(function()
  if M.save_in_progress then return false end
  M.save_in_progress = true
  logger.debug 'save_timers: triggered.'

  local timers_to_save = get_snapshot()
  local json_content = vim.json.encode(timers_to_save)

  if not json_content or json_content == '' then
    M.save_in_progress = false
    return false
  end

  local dir = get_dir(M.config.file)
  local _, mkdir_err = FS.mkdirp(dir):await()
  if mkdir_err then
    logger.debug('save_timers: Failed to create directory: ' .. tostring(mkdir_err))
    M.save_in_progress = false
    return false
  end

  local temp_file = M.config.file .. '.tmp.' .. M.instance_id
  local _, write_err = FS.writefile(temp_file, json_content):await()
  if write_err then
    logger.debug('save_timers: Failed to write temp file: ' .. tostring(write_err))
    M.save_in_progress = false
    return false
  end

  local _, rename_err = FS.rename(temp_file, M.config.file):await()
  if rename_err then
    logger.debug('save_timers: Failed to rename temp file: ' .. tostring(rename_err))
    FS.unlink(temp_file):await()
    M.save_in_progress = false
    return false
  end

  M.last_save_time = now()
  M.save_in_progress = false
  return true
end)

M.validate = function(config)
  if config.scope then
    local valid_scopes = { 'workspace', 'file', 'filetype', 'global' }
    local valid = false
    for _, scope in ipairs(valid_scopes) do
      if config.scope == scope then
        valid = true
        break
      end
    end
    if not valid then
      return 'Invalid scope value, must be \'workspace\', \'file\', \'filetype\' or \'global\''
    end
  end

  if config.file and type(config.file) ~= 'string' then
    return 'Invalid file value, must be a string'
  end

  if not config.file:lower():match '%.json' then
    return 'Invalid file value, must be a JSON file (ending with .json)'
  end

  if config.mode then
    local valid_modes = { 'active', 'idle', 'all' }
    local ok = false
    for _, m in ipairs(valid_modes) do
      if config.mode == m then
        ok = true
        break
      end
    end
    if not ok then return 'Invalid mode value, must be \'active\', \'idle\', or \'all\'' end
  end

  if config.save_on then
    if type(config.save_on) ~= 'table' then
      return 'Invalid save_on value, must be a table of strings'
    end
    local valid_triggers = { 'exit', 'periodic', 'focus_change' }
    for _, trig in ipairs(config.save_on) do
      local ok = false
      for _, v in ipairs(valid_triggers) do
        if trig == v then
          ok = true
          break
        end
      end
      if not ok then
        return 'Invalid save_on trigger \''
          .. tostring(trig)
          .. '\', must be one of: \'exit\', \'periodic\', \'focus_change\''
      end
    end
  end

  if
    config.save_interval and (type(config.save_interval) ~= 'number' or config.save_interval <= 0)
  then
    return 'Invalid save_interval value, must be a positive number'
  end
end

local function should_save(trigger)
  for _, t in ipairs(M.config.save_on) do
    if t == trigger then return true end
  end
  return false
end

M.setup = function(config)
  if config then
    config = vim.tbl_deep_extend('force', M.config, config)

    local err = M.validate(config)
    if err then
      error(err, 0)
    else
      M.config = config
    end
  end

  M.instance_id = generate_instance_id()
  logger.debug('PersistentTimer initializing. Instance ID: ' .. M.instance_id)

  local group = vim.api.nvim_create_augroup('CordPersistentTimerExtension', { clear = true })

  if should_save 'exit' then
    vim.api.nvim_create_autocmd('VimLeavePre', {
      group = group,
      callback = function()
        logger.debug 'VimLeavePre: Triggering final save.'
        local done = false
        Async.run(function()
          local timers_to_save = get_snapshot()
          local json_content = vim.json.encode(timers_to_save)
          if json_content and json_content ~= '' then
            local dir = get_dir(M.config.file)
            local _, mkdir_err = FS.mkdirp(dir):await()
            if not mkdir_err then
              local _, write_err = FS.writefile(M.config.file, json_content):await()
              if write_err then
                logger.debug('VimLeavePre: Final save failed: ' .. tostring(write_err))
              end
            else
              logger.debug(
                'VimLeavePre: Failed to create dir for final save: ' .. tostring(mkdir_err)
              )
            end
          end
          done = true
        end)
        vim.wait(2000, function() return done end, 10)
      end,
    })
  end

  if should_save 'focus_change' then
    vim.api.nvim_create_autocmd('FocusLost', {
      group = group,
      callback = function()
        if now() - M.last_save_time >= 5 then Async.run(function() save_timers():await() end) end
      end,
    })
  end

  if should_save 'periodic' then
    local timer = uv.new_timer()
    if timer then
      timer:start(M.config.save_interval * 1000, M.config.save_interval * 1000, function()
        if now() - M.last_save_time >= M.config.save_interval then
          Async.run(function() save_timers():await() end)
        end
      end)
    end
  end

  return {
    name = 'PersistentTimer',
    description = 'Persistently track time spent scope-wise across sessions',
    hooks = {
      post_activity = {
        fun = Async.wrap(function(opts, activity)
          if not M.is_initialized then
            M.is_initialized = true
            logger.debug 'post_activity: First run, performing initial load.'
            load_timers():await()
            logger.debug 'post_activity: Initial load complete.'
          end

          local key = get_key(opts)
          if not key then return end

          local mode_to_run = (M.config.mode == 'active' and not opts.is_idle and 'active')
            or (M.config.mode == 'idle' and opts.is_idle and 'idle')
            or (M.config.mode == 'all' and 'all')

          if not mode_to_run then
            if M.current_key then
              stop_timer(M.current_key)
              M.current_key = nil
            end
            return
          end

          if M.current_key ~= key then
            logger.debug('Key changed from "' .. tostring(M.current_key) .. '" to "' .. key .. '"')
            stop_timer(M.current_key)
            start_timer(key, mode_to_run)
          end

          local timer = ensure_key(key)
          local bucket = timer.modes[M.config.mode] or timer.modes.all

          if not bucket.display_start_time then
            bucket.display_start_time = bucket.session_start_time - bucket.total_time
          end

          activity.timestamps = activity.timestamps or {}
          activity.timestamps.start = bucket.display_start_time
        end),
        priority = 10,
      },
    },
  }
end

return M
