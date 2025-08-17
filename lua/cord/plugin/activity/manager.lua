local async = require 'cord.core.async'
local activities = require 'cord.plugin.activity'
local ws_utils = require 'cord.plugin.fs.workspace'
local config_utils = require 'cord.plugin.config.util'
local hooks = require 'cord.plugin.activity.hooks'
local config = require 'cord.plugin.config'
local logger = require 'cord.plugin.log'

local uv = vim.loop or vim.uv

---@class ActivityManager
---@field config CordConfig Configuration options
---@field tx table Event transmitter instance
---@field is_focused boolean Whether Neovim is focused
---@field is_paused boolean Whether activity updates are paused
---@field is_force_idle boolean Whether idle state is forced
---@field events_enabled boolean Whether events are enabled
---@field last_activity Activity Last activity state
---@field opts CordOpts Latest options passed to activity updates
---@field last_opts? CordOpts Previous options passed to activity updates, used to detect changes
---@field workspace_dir string Current workspace directory
---@field repo_url string|nil Current Git repository URL
---@field workspace_cache table Cache of workspace directories, their names and repositories
local ActivityManager = {}
local mt = { __index = ActivityManager }

---@class Activity
---@field type? string One of 'playing', 'listening', 'watching', 'competing'
---@field status_display_type? string One of 'name', 'state', 'details'. Controls which field is displayed in the user's status text in the member list
---@field details? string Detailed information about what the user is doing
---@field details_url? string URL for the details field to make it clickable
---@field state? string Secondary information about what the user is doing
---@field state_url? string URL for the state field to make it clickable
---@field timestamps? ActivityTimestamps Contains `start` and `end` timestamps for the activity
---@field assets? ActivityAssets Defines images and tooltips, including `large_image`, `large_text`, `large_url`, `small_image`, `small_text`, and `small_url`
---@field buttons? CordButtonConfig[] Array of objects, each with `label` and `url`, defining interactive buttons in the presence
---@field is_idle? boolean Whether the activity should be considered as idle

---@class ActivityTimestamps
---@field start? integer Start timestamp in milliseconds
---@field end? integer End timestamp in milliseconds

---@class ActivityAssets
---@field large_image? string Large image ID or URL
---@field large_text? string Large image text
---@field large_url? string URL for the large image to make it clickable
---@field small_image? string Small image ID or URL
---@field small_text? string Small image text
---@field small_url? string URL for the small image to make it clickable

---@class CordOpts
---@field manager ActivityManager Reference to the ActivityManager instance
---@field name? string Name associated with the current mapping
---@field tooltip? string Tooltip associated with the current mapping
---@field filename string Current buffer's filename
---@field filetype string Current buffer's filetype
---@field is_read_only boolean Whether the current buffer is read-only
---@field cursor_line integer Current cursor line
---@field cursor_char integer Current cursor character
---@field timestamp number Timestamp passed to the Rich Presence in milliseconds
---@field workspace? string Current workspace name
---@field workspace_dir? string Current workspace directory
---@field repo_url? string Current Git repository URL, if any
---@field is_focused boolean Whether Neovim is focused
---@field is_idle boolean Whether the session is idle
---@field buttons? CordButtonConfig[] Buttons configuration
---@field type string Which category the asset belongs to, e.g. 'language' or 'docs'
---@field icon? string Asset icon URL or name, if any
---@field text? string Asset text, if any

local has_initialized = false

---Meant to be called only once throughout the entire lifetime of the program
---@return nil
local function setup()
  logger.debug 'ActivityManager.setup: initializing hooks and plugin API'
  if config.hooks then
    for event, hook in pairs(config.hooks) do
      if type(hook) == 'function' then
        hooks.register(event, hook, 200)
      elseif type(hook) == 'table' then
        hooks.register(event, hook[1] or hook.fun, hook.priority or 200)
      end
    end
  end

  return require('cord.api.plugin').init()
end

---Create a new ActivityManager instance
---@param opts {config: CordConfig, tx: table} Configuration and transmitter options
ActivityManager.new = async.wrap(function(opts)
  local self = setmetatable({
    tx = opts.tx,
    is_focused = true,
    is_paused = false,
    is_force_idle = false,
    events_enabled = true,
    workspace_cache = {},
  }, mt)

  local rawdir = vim.fn.expand '%:p'
  local dir = ws_utils.find(rawdir):get() or vim.fn.getcwd()
  self.workspace_dir = dir
  logger.debug(function() return 'ActivityManager.new: workspace_dir=' .. tostring(dir) end)

  local cache = {
    dir = dir,
    name = vim.fn.fnamemodify(dir, ':t'),
  }

  local repo_url = ws_utils.find_git_repository(dir):get()
  if repo_url then
    self.repo_url = repo_url
    cache.repo_url = repo_url
    logger.trace(function() return 'ActivityManager.new: repo_url=' .. tostring(repo_url) end)
  end

  self.workspace_cache[vim.fn.fnamemodify(rawdir, ':h')] = cache

  if not has_initialized then
    local err = setup()
    if err then
      logger.notify(err, vim.log.levels.ERROR)
      error('Failed to initialize ActivityManager', 0)
    end
    has_initialized = true
  end

  return self
end)

---Run the activity manager
---@return nil
function ActivityManager:run()
  logger.debug 'ActivityManager.run: starting'
  self.workspace = vim.fn.fnamemodify(self.workspace_dir, ':t')
  self.last_updated = uv.now()

  hooks.run('ready', self)

  self:queue_update(true)
  if config.advanced.plugin.autocmds then self.setup_autocmds() end
  logger.trace(
    function() return 'ActivityManager.run: autocmds=' .. tostring(config.advanced.plugin.autocmds) end
  )

  if config.idle.enabled then
    logger.trace(
      function()
        return 'ActivityManager.run: idle timer enabled; timeout=' .. tostring(config.idle.timeout)
      end
    )
    self.idle_timer = uv.new_timer()
    self.idle_timer:start(
      0,
      config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end
end

---Clean up the activity manager
---@return nil
function ActivityManager:cleanup()
  logger.debug 'ActivityManager.cleanup'
  self:clear_autocmds()
  if self.idle_timer then
    self.idle_timer:stop()
    if not self.idle_timer:is_closing() then self.idle_timer:close() end
  end
end

---Check if an activity update is needed
---@return boolean Whether an update is needed
function ActivityManager:should_update()
  local should_update = not self.last_opts
    or self.opts.filename ~= self.last_opts.filename
    or self.opts.filetype ~= self.last_opts.filetype
    or self.opts.is_read_only ~= self.last_opts.is_read_only
    or self.opts.cursor_line ~= self.last_opts.cursor_line
    or self.opts.cursor_char ~= self.last_opts.cursor_char
    or self.opts.is_focused ~= self.last_opts.is_focused

  logger.trace(function() return 'ActivityManager.should_update=' .. tostring(should_update) end)
  return should_update
end

---Queue an activity update
---@param force_update? boolean Whether to force the update regardless of conditions
function ActivityManager:queue_update(force_update)
  if not self.events_enabled then return end

  self.opts = self:build_opts()
  if not self.is_force_idle and (force_update or self:should_update()) then
    logger.debug(
      function()
        return 'ActivityManager.queue_update: running update; force=' .. tostring(force_update)
      end
    )
    if self.is_idle then hooks.run('idle_leave', self.opts) end
    self:update_activity()
  end
end

---Check if the activity should be updated to idle
---@return nil
function ActivityManager:check_idle()
  if not self.events_enabled then return end
  if not config.idle.enabled and not self.is_force_idle then return end
  if self.is_idle then return end

  local time_elapsed = uv.now() - self.last_updated
  logger.trace(
    function()
      return 'ActivityManager.check_idle: elapsed='
        .. tostring(time_elapsed)
        .. ', timeout='
        .. tostring(config.idle.timeout)
        .. ', is_force_idle='
        .. tostring(self.is_force_idle)
        .. ', is_focused='
        .. tostring(self.is_focused)
    end
  )
  if
    self.is_force_idle
    or (time_elapsed >= config.idle.timeout and (config.idle.ignore_focus or not self.is_focused))
  then
    logger.debug 'ActivityManager.check_idle: switching to idle activity'
    self:update_idle_activity()
  else
    local time_remaining = config.idle.timeout - time_elapsed
    self.idle_timer:stop()
    self.idle_timer:start(
      time_remaining,
      0,
      vim.schedule_wrap(function()
        self.idle_timer:start(
          0,
          config.idle.timeout,
          vim.schedule_wrap(function() self:check_idle() end)
        )
      end)
    )
  end
end

---Update the activity to idle
---@return nil
function ActivityManager:update_idle_activity()
  if not self.opts then self.opts = self:build_opts() end
  self.opts.is_idle = true
  self.is_idle = true
  self.last_updated = uv.now()
  logger.debug 'ActivityManager.update_idle_activity'

  if config.idle.show_status then
    local buttons = config_utils.get_buttons(self.opts)
    self.opts.buttons = buttons
    if config.timestamp.enabled and config.timestamp.reset_on_idle then
      self.opts.timestamp = os.time()
    end

    local activity = activities.build_idle_activity(self.opts)

    hooks.run('post_activity', self.opts, activity)

    if config.timestamp.shared then
      self.opts.timestamp = nil
      self.last_opts.timestamp = nil
    end

    if self.should_skip_update then
      self.should_skip_update = false
      return
    end

    self.tx:update_activity(activity)
    hooks.run('idle_enter', self.opts)
    logger.trace(
      function() return 'ActivityManager.update_idle_activity: activity=' .. vim.inspect(activity) end
    )
  else
    logger.trace 'ActivityManager.update_idle_activity: clear activity (no idle status)'
    hooks.run('post_activity', self.opts)

    if config.timestamp.shared then
      self.opts.timestamp = nil
      self.last_opts.timestamp = nil
    end

    if self.should_skip_update then
      self.should_skip_update = false
      return
    end

    self.tx:clear_activity()
    hooks.run('idle_enter', self.opts)
  end
end

---Update the activity
---@return nil
function ActivityManager:update_activity()
  logger.debug 'ActivityManager.update_activity'
  self.is_idle = false
  self.is_force_idle = false
  self.last_opts = self.opts
  self.last_updated = uv.now()
  self.opts.is_idle = false

  hooks.run('pre_activity', self.opts)

  local activity = activities.build_activity(self.opts)
  if activity == true then return end
  if activity == false then return self:clear_activity() end

  hooks.run('post_activity', self.opts, activity)

  if config.timestamp.shared then
    self.opts.timestamp = nil
    self.last_opts.timestamp = nil
  end

  if self.should_skip_update then
    self.should_skip_update = false
    return
  end

  self.tx:update_activity(activity)
  logger.trace(
    function() return 'ActivityManager.update_activity: activity=' .. vim.inspect(activity) end
  )
end

---Skip the next activity update
---@return nil
function ActivityManager:skip_update() self.should_skip_update = true end

---Pause activity updates and events
---@return nil
function ActivityManager:pause()
  if self.is_paused then return end

  self:pause_events()
  if self.idle_timer then self.idle_timer:stop() end
  self.is_paused = true
  logger.debug 'ActivityManager.pause'
end

---Resume activity updates and events
---@return nil
function ActivityManager:resume()
  if not self.is_paused then return end

  self:resume_events()
  if self.idle_timer then
    self.idle_timer:stop()
    self.idle_timer:start(
      0,
      config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end
  self.is_paused = false
  logger.debug 'ActivityManager.resume'
end

---Pause only events, keeping the current activity state
---@return nil
function ActivityManager:pause_events() self.events_enabled = false end

---Resume events after they were paused
---@return nil
function ActivityManager:resume_events()
  self.events_enabled = true
  self:queue_update(true)
  logger.trace 'ActivityManager.resume_events'
end

---Set idle state
---@return nil
function ActivityManager:idle() self:update_idle_activity() end

---Force idle state
---@return nil
function ActivityManager:force_idle()
  self.is_force_idle = true
  self:update_idle_activity()
  logger.trace 'ActivityManager.force_idle'
end

---Unforce idle state
---@return nil
function ActivityManager:unidle()
  self.is_force_idle = false
  self:queue_update(true)
  logger.trace 'ActivityManager.unidle'
end

---Toggle idle state
---@return nil
function ActivityManager:toggle_idle(force)
  if self.is_force_idle or self.is_idle then
    self:unidle()
  elseif force == true then
    self:force_idle()
  else
    self:idle()
  end
  logger.trace(function() return 'ActivityManager.toggle_idle: force=' .. tostring(force) end)
end

---Hide the activity
---@return nil
function ActivityManager:hide()
  self:pause()
  self:clear_activity(true)
  logger.debug 'ActivityManager.hide'
end

---Suppress the activity for current Neovim instance
---@return nil
function ActivityManager:suppress()
  self:pause()
  self:clear_activity()
  logger.debug 'ActivityManager.suppress'
end

---Toggle the activity
---@return nil
function ActivityManager:toggle()
  if self.is_paused then
    self:resume()
  else
    self:hide()
  end
  logger.trace 'ActivityManager.toggle'
end

---Toggle suppress state
---@return nil
function ActivityManager:toggle_suppress()
  if self.is_paused then
    self:resume()
  else
    self:suppress()
  end
  logger.trace 'ActivityManager.toggle_suppress'
end

---Setup autocmds
---@return nil
function ActivityManager.setup_autocmds()
  logger.trace 'ActivityManager.setup_autocmds'
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
      autocmd BufEnter * lua require'cord.server'.manager:on_buf_enter()
      autocmd TermOpen * lua require'cord.server'.manager:on_buf_enter()
      autocmd FocusGained * lua require'cord.server'.manager:on_focus_gained()
      autocmd FocusLost * lua require'cord.server'.manager:on_focus_lost()
    augroup END
  ]]

  if config.advanced.plugin.cursor_update == 'on_hold' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorHold,CursorHoldI * lua require'cord.server'.manager:on_cursor_update()
      augroup END
    ]]
  elseif config.advanced.plugin.cursor_update == 'on_move' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorMoved,CursorMovedI * lua require'cord.server'.manager:on_cursor_update()
      augroup END
    ]]
  end
end

---Clear autocmds
---@return nil
function ActivityManager.clear_autocmds()
  logger.trace 'ActivityManager.clear_autocmds'
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
    augroup END
  ]]
end

---Set the activity
---@param activity table Activity to set
---@return nil
function ActivityManager:set_activity(activity) self.tx:update_activity(activity) end

---Clear the activity
---@param force? boolean Whether to force clear the activity
---@return nil
function ActivityManager:clear_activity(force) self.tx:clear_activity(force) end

---Check if the time should be updated
---@return boolean Whether the time should be updated
function ActivityManager:should_update_time()
  return config.timestamp.enabled
      and (config.timestamp.reset_on_change or config.timestamp.reset_on_idle and self.is_idle)
    or false
end

---Handle buffer enter event
---@return nil
function ActivityManager:on_buf_enter()
  logger.trace 'ActivityManager.on_buf_enter'
  local rawdir = vim.fn.expand '%:p'
  local cached = self.workspace_cache[vim.fn.fnamemodify(rawdir, ':h')]
  if cached then
    if cached.dir ~= self.workspace_dir then
      self.workspace_dir = cached.dir
      self.workspace = cached.name
      self.repo_url = cached.repo_url
      self.opts.workspace_dir = self.workspace_dir
      self.opts.workspace = self.workspace
      self.opts.repo_url = self.repo_url

      hooks.run('workspace_change', self.opts)
    end

    self:queue_update()
    return
  elseif cached == false then
    logger.trace 'ActivityManager.on_buf_enter: cached=false; clearing workspace'
    if self.workspace_dir then
      self.workspace_dir = nil
      self.workspace = nil
      self.repo_url = nil
      self.opts.workspace_dir = nil
      self.opts.workspace = nil
      self.opts.repo_url = nil

      hooks.run('workspace_change', self.opts)
    end

    self:queue_update()
    return
  end

  async.run(function()
    local dir = ws_utils.find(rawdir):get() or vim.fn.getcwd()
    logger.trace(
      function() return 'ActivityManager.on_buf_enter: detected dir=' .. tostring(dir) end
    )

    if not dir then
      self.workspace_cache[vim.fn.fnamemodify(rawdir, ':h')] = false
      self:queue_update()
      return
    end

    self.workspace_dir = dir
    self.workspace = vim.fn.fnamemodify(self.workspace_dir, ':t')
    self.opts.workspace_dir = self.workspace_dir
    self.opts.workspace = self.workspace

    local repo_url = ws_utils.find_git_repository(dir):get()
    self.repo_url = repo_url
    self.opts.repo_url = repo_url

    self.workspace_cache[vim.fn.fnamemodify(rawdir, ':h')] = {
      dir = self.workspace_dir,
      name = self.workspace,
      repo_url = self.repo_url,
    }

    hooks.run('workspace_change', self.opts)

    self:queue_update()
  end)
end

---Handle focus gained event
---@return nil
function ActivityManager:on_focus_gained()
  if not self.events_enabled then return end
  self.is_focused = true
  self.opts.is_focused = true
  logger.trace 'ActivityManager.on_focus_gained'

  if config.idle.unidle_on_focus then self:queue_update(true) end
end

---Handle focus lost event
---@return nil
function ActivityManager:on_focus_lost()
  if not self.events_enabled then return end
  self.is_focused = false
  self.opts.is_focused = false
  logger.trace 'ActivityManager.on_focus_lost'
end

---Handle cursor update event
---@return nil
function ActivityManager:on_cursor_update()
  if not self.events_enabled then return end
  logger.trace 'ActivityManager.on_cursor_update'
  self:queue_update()
end

---Build options
---@return CordOpts
function ActivityManager:build_opts()
  logger.trace 'ActivityManager.build_opts'
  local cursor_position = vim.api.nvim_win_get_cursor(0)
  local opts = {
    manager = self,
    filename = vim.fn.expand '%:t',
    filetype = vim.bo.filetype,
    buftype = vim.bo.buftype,
    is_read_only = vim.bo.readonly or not vim.bo.modifiable,
    cursor_line = cursor_position[1],
    cursor_char = cursor_position[2],
    workspace_dir = self.workspace_dir,
    workspace = self.workspace,
    repo_url = self.repo_url,
    is_focused = self.is_focused,
    is_idle = self.is_idle,
  }
  if config.timestamp.enabled and not config.timestamp.shared then
    if self.last_opts and self.last_opts.timestamp then
      opts.timestamp = self.last_opts.timestamp
    else
      opts.timestamp = os.time()
    end
  end
  if self:should_update_time() then opts.timestamp = os.time() end
  local buttons = config_utils.get_buttons(opts)
  opts.buttons = buttons

  return opts
end

return ActivityManager
