local async = require 'cord.core.async'
local builder = require 'cord.internal.activity.builder'
local ws_utils = require 'cord.internal.activity.workspace'
local hooks = require 'cord.internal.hooks'
local config = require 'cord.api.config'
local logger = require 'cord.api.log'

local uv = vim.loop or vim.uv

--------------------------------------------------------------------------------
-- Type Definitions
--------------------------------------------------------------------------------

---@class Activity
---@field status_display_type? string One of 'name', 'state', 'details'. Controls which field is displayed in the user's status text in the member list
---@field details? string Detailed information about what the user is doing
---@field details_url? string URL for the details field
---@field state? string Secondary information about what the user is doing
---@field state_url? string URL for the state field
---@field timestamps? ActivityTimestamps Contains `start` and `end` timestamps
---@field assets? ActivityAssets Defines images and tooltips
---@field buttons? CordButtonConfig[] Array of button objects
---@field is_idle? boolean Whether the activity should be considered as idle

---@class ActivityTimestamps
---@field start? integer Start timestamp in milliseconds
---@field end? integer End timestamp in milliseconds

---@class ActivityAssets
---@field large_image? string Large image ID or URL
---@field large_text? string Large image text
---@field large_url? string URL for the large image
---@field small_image? string Small image ID or URL
---@field small_text? string Small image text
---@field small_url? string URL for the small image

---@class CordOpts
---@field manager ActivityManager Reference to the ActivityManager instance
---@field cache CordCache Global cache instance
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
---@field type string Which category the asset belongs to
---@field icon? string Asset icon URL or name
---@field text? string Asset text

--------------------------------------------------------------------------------
-- Utility Functions
--------------------------------------------------------------------------------

---Resolve a value that may be a function or static
---@generic T
---@param value T|fun(...): T
---@param ... any Arguments to pass if value is a function
---@return T
local function resolve(value, ...)
  local is_async = async.is_async(value)
  if type(value) == 'function' or is_async then
    if is_async then return value(...):await() end
    return value(...)
  end
  return value
end

---Get current buffer's directory info
---@return string rawdir The expanded path
---@return string parent The parent directory
local function get_buffer_dir()
  local rawdir = vim.fn.expand '%:p'
  return rawdir, vim.fn.fnamemodify(rawdir, ':h')
end

--------------------------------------------------------------------------------
-- Button configuration
--------------------------------------------------------------------------------

---@class ButtonBuilder
local ButtonBuilder = {}

---Build buttons array from config
---@param opts CordOpts
---@return CordButtonConfig[]
function ButtonBuilder.build(opts)
  if not config.buttons then return {} end

  local buttons = {}
  for _, source in ipairs(config.buttons) do
    local label = resolve(source.label, opts)
    local url = resolve(source.url, opts)
    if label and url then buttons[#buttons + 1] = { label = label, url = url } end
  end
  return buttons
end

--------------------------------------------------------------------------------
-- Workspace caching
--------------------------------------------------------------------------------

local Cache = require 'cord.core.cache'

---@class WorkspaceInfo
---@field dir string Workspace directory
---@field name string Workspace name
---@field repo_url? string Git repository URL

---@class WorkspaceCache
---@field inner CordCache
---@field current WorkspaceInfo|nil
local WorkspaceCache = {}
WorkspaceCache.__index = WorkspaceCache

function WorkspaceCache.new()
  return setmetatable({ inner = Cache.new(), current = nil }, WorkspaceCache)
end

---@param parent string
---@return WorkspaceInfo|false|nil
function WorkspaceCache:get(parent) return self.inner:get(parent) end

---@param parent string
---@param info WorkspaceInfo|false
function WorkspaceCache:set(parent, info) self.inner:set(parent, info) end

---@param info WorkspaceInfo|nil
---@return boolean changed
function WorkspaceCache:set_current(info)
  local changed = not self.current
    or (info and self.current.dir ~= info.dir)
    or (not info and self.current ~= nil)
  self.current = info
  return changed
end

---@param rawdir string
---@return WorkspaceInfo
function WorkspaceCache.discover(rawdir)
  local dir = ws_utils.find(rawdir):await() or vim.fn.getcwd()
  return {
    dir = dir,
    name = vim.fn.fnamemodify(dir, ':t'),
    repo_url = ws_utils.find_git_repository(dir):await(),
  }
end

--------------------------------------------------------------------------------
-- Idle state handling
--------------------------------------------------------------------------------

---@class IdleTimer
---@field timer any
---@field on_idle fun()
---@field is_idle boolean
---@field is_forced boolean
---@field last_activity number
local IdleTimer = {}
IdleTimer.__index = IdleTimer

---@param on_idle fun()
---@return IdleTimer
function IdleTimer.new(on_idle)
  return setmetatable({
    timer = nil,
    is_idle = false,
    is_forced = false,
    last_activity = uv.now(),
    on_idle = on_idle,
  }, IdleTimer)
end

function IdleTimer:start()
  if not config.idle.enabled then return end
  logger.trace(function() return 'IdleTimer: starting; timeout=' .. config.idle.timeout end)
  self.timer = uv.new_timer()
  self.timer:start(0, config.idle.timeout, function() self:check() end)
end

function IdleTimer:stop()
  if self.timer then self.timer:stop() end
end

function IdleTimer:cleanup()
  if not self.timer then return end
  self.timer:stop()
  if not self.timer:is_closing() then self.timer:close() end
  self.timer = nil
end

function IdleTimer:record_activity() self.last_activity = uv.now() end

---@param is_focused? boolean
function IdleTimer:check(is_focused)
  if self.is_idle then return end
  if not config.idle.enabled and not self.is_forced then return end

  local elapsed = uv.now() - self.last_activity
  local should_idle = self.is_forced
    or (elapsed >= config.idle.timeout and (config.idle.ignore_focus or not is_focused))

  logger.trace(
    function()
      return string.format(
        'IdleTimer.check: elapsed=%d, should_idle=%s',
        elapsed,
        tostring(should_idle)
      )
    end
  )

  if should_idle then
    logger.debug 'IdleTimer: entering idle'
    self.is_idle = true
    vim.schedule(self.on_idle)
  else
    self:reschedule(config.idle.timeout - elapsed)
  end
end

---@param remaining number
function IdleTimer:reschedule(remaining)
  if not self.timer then return end
  self.timer:stop()
  self.timer:start(remaining, 0, function()
    self.timer:start(0, config.idle.timeout, function() self:check() end)
  end)
end

function IdleTimer:force()
  self.is_forced = true
  self.is_idle = true
  self.on_idle()
  logger.trace 'IdleTimer: forced'
end

function IdleTimer:leave()
  self.is_idle = false
  self.is_forced = false
end

function IdleTimer:reset()
  self:leave()
  if not self.timer then return end
  self.timer:stop()
  self.timer:start(0, config.idle.timeout, function() self:check() end)
end

--------------------------------------------------------------------------------
-- Autocmds
--------------------------------------------------------------------------------

---@class AutocmdController
local AutocmdController = {}
AutocmdController.__index = AutocmdController

function AutocmdController.new() return setmetatable({}, AutocmdController) end

function AutocmdController.setup()
  logger.trace 'AutocmdController: setup'
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
      autocmd BufEnter * lua require'cord.server'.manager:on_buf_enter()
      autocmd TermOpen * lua require'cord.server'.manager:on_buf_enter()
      autocmd FocusGained * lua require'cord.server'.manager:on_focus_gained()
      autocmd FocusLost * lua require'cord.server'.manager:on_focus_lost()
    augroup END
  ]]

  local cursor_mode = config.advanced.plugin.cursor_update
  if cursor_mode == 'on_hold' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorHold,CursorHoldI * lua require'cord.server'.manager:on_cursor_update()
      augroup END
    ]]
  elseif cursor_mode == 'on_move' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorMoved,CursorMovedI * lua require'cord.server'.manager:on_cursor_update()
      augroup END
    ]]
  end
end

function AutocmdController.clear()
  logger.trace 'AutocmdController: clear'
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
    augroup END
  ]]
end

--------------------------------------------------------------------------------
-- Update debouncing
--------------------------------------------------------------------------------

---@class UpdateDebouncer
---@field timer any Timer handle
---@field is_active boolean Whether the timer is currently running
---@field timer_target number When the timer will fire (ms timestamp)
---@field last_update number Last update timestamp (ms)
---@field pending_force boolean|nil Whether pending update should be forced
---@field in_delay_phase boolean Whether we've already rescheduled for settling
local UpdateDebouncer = {}
UpdateDebouncer.__index = UpdateDebouncer

---@return UpdateDebouncer
function UpdateDebouncer.new()
  return setmetatable({
    timer = uv.new_timer(),
    is_active = false,
    timer_target = 0,
    last_update = 0,
    pending_force = nil,
    in_delay_phase = false,
  }, UpdateDebouncer)
end

---@param duration number Duration in ms
---@param callback fun(force: boolean|nil)
function UpdateDebouncer:schedule(duration, callback)
  self.timer:stop()
  self.is_active = true
  self.timer:start(duration, 0, function() self:execute(callback) end)
end

---Request a debounced update
---@param force boolean|nil Whether to force the update
---@param callback fun(force: boolean|nil) Function to call when update should execute
function UpdateDebouncer:request(force, callback)
  local debounce = config.advanced.plugin.debounce
  local delay = debounce and debounce.delay or 0
  local interval = debounce and debounce.interval or 0

  if delay <= 0 and interval <= 0 then
    callback(force)
    return
  end

  self.pending_force = self.pending_force or force

  local now = uv.now()

  if self.is_active then
    local time_until_fire = self.timer_target - now

    if delay > 0 and time_until_fire < delay and not self.in_delay_phase then
      self.timer_target = now + delay
      self.in_delay_phase = true
      logger.trace(
        function() return 'UpdateDebouncer: near-fire burst, rescheduling for ' .. delay .. 'ms' end
      )
      self:schedule(delay, callback)
    else
      logger.trace 'UpdateDebouncer: timer running, coalescing'
    end
    return
  end

  local elapsed = now - self.last_update

  if interval > 0 and elapsed < interval then
    local remaining = interval - elapsed
    self.timer_target = now + remaining
    self.in_delay_phase = false
    logger.trace(
      function() return 'UpdateDebouncer: throttling, scheduling in ' .. remaining .. 'ms' end
    )
    self:schedule(remaining, callback)
    return
  end

  if delay > 0 then
    self.timer_target = now + delay
    self.in_delay_phase = true
    logger.trace(function() return 'UpdateDebouncer: delaying for ' .. delay .. 'ms' end)
    self:schedule(delay, callback)
    return
  end

  self:execute(callback)
end

---@param callback fun(force: boolean|nil)
function UpdateDebouncer:execute(callback)
  self.last_update = uv.now()
  local was_forced = self.pending_force
  self.pending_force = nil
  self.is_active = false
  self.in_delay_phase = false
  vim.schedule(function() callback(was_forced) end)
end

---Cancel any pending update
function UpdateDebouncer:cancel()
  self.timer:stop()
  self.is_active = false
  self.pending_force = nil
  self.in_delay_phase = false
end

---Cleanup resources
function UpdateDebouncer:cleanup()
  self.timer:stop()
  if not self.timer:is_closing() then self.timer:close() end
end

--------------------------------------------------------------------------------
-- Options builder
--------------------------------------------------------------------------------

---@class OptionsBuilder
---@field manager ActivityManager
local OptionsBuilder = {}
OptionsBuilder.__index = OptionsBuilder

---@param manager ActivityManager
---@return OptionsBuilder
function OptionsBuilder.new(manager) return setmetatable({ manager = manager }, OptionsBuilder) end

---@return boolean
function OptionsBuilder:should_reset_timestamp()
  if not config.timestamp.enabled then return false end
  return config.timestamp.reset_on_change
    or (config.timestamp.reset_on_idle and self.manager.idle_timer.is_idle)
    or false
end

function OptionsBuilder:build_base()
  logger.trace 'OptionsBuilder.build_base'

  local mgr = self.manager
  local cursor = vim.api.nvim_win_get_cursor(0)
  local ws = mgr.workspace.current

  return {
    manager = mgr,
    cache = mgr.opts_cache,
    filename = vim.fn.expand '%:t',
    filetype = vim.bo.filetype,
    buftype = vim.bo.buftype,
    is_read_only = vim.bo.readonly or not vim.bo.modifiable,
    cursor_line = cursor[1],
    cursor_char = cursor[2],
    workspace_dir = ws and ws.dir or vim.fn.getcwd(),
    workspace = ws and ws.name or vim.fn.fnamemodify(vim.fn.getcwd(), ':t'),
    repo_url = ws and ws.repo_url,
    is_focused = mgr.is_focused,
    is_idle = mgr.idle_timer.is_idle,
  }
end

---@return CordOpts
---@param full boolean?
function OptionsBuilder:build(full)
  logger.trace 'OptionsBuilder.build'

  local mgr = self.manager
  local opts = self:build_base()

  if full == false then
    opts.timestamp = mgr.opts and mgr.opts.timestamp
      or mgr.last_opts and mgr.last_opts.timestamp
      or nil

    opts.buttons = mgr.opts and mgr.opts.buttons or mgr.last_opts and mgr.last_opts.buttons or nil
  else
    if config.timestamp.enabled and not config.timestamp.shared then
      opts.timestamp = (mgr.last_opts and mgr.last_opts.timestamp) or os.time()
    end
    if self:should_reset_timestamp() then opts.timestamp = os.time() end

    opts.buttons = ButtonBuilder.build(opts)
  end

  return opts
end

---@param curr CordOpts
---@param prev CordOpts|nil
---@return boolean
function OptionsBuilder.has_changed(curr, prev)
  if not prev then return true end
  return curr.filename ~= prev.filename
    or curr.filetype ~= prev.filetype
    or curr.is_read_only ~= prev.is_read_only
    or curr.cursor_line ~= prev.cursor_line
    or curr.cursor_char ~= prev.cursor_char
    or curr.is_focused ~= prev.is_focused
end

--------------------------------------------------------------------------------
-- Activity update logic
--------------------------------------------------------------------------------

---@class ActivityUpdater
---@field manager ActivityManager
local ActivityUpdater = {}
ActivityUpdater.__index = ActivityUpdater

---@param manager ActivityManager
---@return ActivityUpdater
function ActivityUpdater.new(manager) return setmetatable({ manager = manager }, ActivityUpdater) end

function ActivityUpdater:update()
  local mgr = self.manager
  logger.debug 'ActivityUpdater.update'

  mgr.idle_timer:leave()
  mgr.idle_timer:record_activity()
  mgr.last_opts = mgr.opts
  mgr.opts.is_idle = false

  hooks.run('pre_activity', mgr.opts)

  local activity = builder.build_activity(mgr.opts)
  if activity == true then return end
  if activity == false then return mgr:clear_activity() end

  mgr:set_activity(activity --[[@as table]])
  logger.trace(function() return 'ActivityUpdater.update: activity=' .. vim.inspect(activity) end)
end

function ActivityUpdater:update_idle()
  local mgr = self.manager
  logger.debug 'ActivityUpdater.update_idle'

  if not mgr.opts then mgr.opts = mgr.options_builder:build() end

  mgr.opts.is_idle = true
  mgr.idle_timer:record_activity()

  if not config.idle.show_status then
    logger.trace 'ActivityUpdater: clearing (no idle status)'
    mgr.tx:clear_activity()
    hooks.run('idle_enter', mgr.opts)
    return
  end

  mgr.opts.buttons = ButtonBuilder.build(mgr.opts)
  if config.timestamp.enabled and config.timestamp.reset_on_idle then
    mgr.opts.timestamp = os.time()
  end

  local activity = builder.build_idle_activity(mgr.opts)
  mgr:set_activity(activity)
  hooks.run('idle_enter', mgr.opts)

  logger.trace(
    function() return 'ActivityUpdater.update_idle: activity=' .. vim.inspect(activity) end
  )
end

--------------------------------------------------------------------------------
-- Event handling
--------------------------------------------------------------------------------

---@class EventHandler
---@field manager ActivityManager
local EventHandler = {}
EventHandler.__index = EventHandler

---@param manager ActivityManager
---@return EventHandler
function EventHandler.new(manager) return setmetatable({ manager = manager }, EventHandler) end

function EventHandler:on_buf_enter()
  local mgr = self.manager
  logger.trace 'EventHandler.on_buf_enter'

  async.run(function()
    hooks.run('buf_enter', mgr)

    local rawdir, parent = get_buffer_dir()
    local cached = mgr.workspace:get(parent)

    if cached then
      if mgr.workspace:set_current(cached) then
        local opts = mgr.options_builder:build(false)
        hooks.run('workspace_change', opts)
      end
      mgr:queue_update()
      return
    end

    if cached == false then
      logger.trace 'EventHandler: cached=false, clearing workspace'
      if mgr.workspace:set_current(nil) then
        local opts = mgr.options_builder:build(false)
        hooks.run('workspace_change', opts)
      end
      mgr:queue_update()
      return
    end

    local info = mgr.workspace.discover(rawdir)
    logger.trace(
      function() return 'EventHandler: discovered dir=' .. tostring(info and info.dir) end
    )

    mgr.workspace:set(parent, info or false)
    if mgr.workspace:set_current(info) then
      local opts = mgr.options_builder:build(false)
      hooks.run('workspace_change', opts)
    end
    mgr:queue_update()
  end)
end

function EventHandler:on_focus_gained()
  local mgr = self.manager
  if not mgr.events_enabled then return end

  mgr.is_focused = true
  if mgr.opts then mgr.opts.is_focused = true end
  logger.trace 'EventHandler.on_focus_gained'

  if config.idle.unidle_on_focus then mgr:queue_update(true) end
end

function EventHandler:on_focus_lost()
  local mgr = self.manager
  if not mgr.events_enabled then return end

  mgr.is_focused = false
  if mgr.opts then mgr.opts.is_focused = false end
  logger.trace 'EventHandler.on_focus_lost'
end

function EventHandler:on_cursor_update()
  local mgr = self.manager
  if not mgr.events_enabled then return end
  logger.trace 'EventHandler.on_cursor_update'
  mgr:queue_update()
end

--------------------------------------------------------------------------------
-- Activity manager
--------------------------------------------------------------------------------

---@class ActivityManager
---@field tx table
---@field is_ready boolean
---@field is_focused boolean
---@field is_paused boolean
---@field events_enabled boolean
---@field opts CordOpts
---@field last_opts CordOpts|nil
---@field should_skip_update boolean
---@field workspace WorkspaceCache
---@field idle_timer IdleTimer
---@field debouncer UpdateDebouncer
---@field autocmds AutocmdController
---@field options_builder OptionsBuilder
---@field activity_updater ActivityUpdater
---@field event_handler EventHandler
---@field opts_cache CordCache
local ActivityManager = {}
ActivityManager.__index = ActivityManager

local has_initialized = false
local has_loaded_workspace = false

---Initialize hooks and plugins (once, globally)
---@return string|nil
local function initialize_global()
  logger.debug 'ActivityManager: initializing hooks and plugins'

  if config.hooks then
    for event, hook in pairs(config.hooks) do
      if type(hook) == 'function' then
        hooks.register(event, hook, 200)
      elseif type(hook) == 'table' then
        hooks.register(event, hook[1] or hook.fun, hook.priority or 200)
      end
    end
  end

  return require('cord.plugins').init():await()
end

---@param opts {tx: table}
---@return ActivityManager
ActivityManager.new = async.wrap(function(opts)
  local self = setmetatable({
    tx = opts.tx,
    is_ready = false,
    is_focused = true,
    is_paused = false,
    events_enabled = true,
    should_skip_update = false,
    workspace = WorkspaceCache.new(),
    autocmds = AutocmdController.new(),
  }, ActivityManager)

  self.idle_timer = IdleTimer.new(function()
    async.run(function() self.activity_updater:update_idle() end)
  end)
  self.debouncer = UpdateDebouncer.new()
  self.options_builder = OptionsBuilder.new(self)
  self.activity_updater = ActivityUpdater.new(self)
  self.event_handler = EventHandler.new(self)
  self.opts_cache = Cache.new()

  local rawdir, parent = get_buffer_dir()
  local info = self.workspace.discover(rawdir)
  self.workspace:set(parent, info)
  self.workspace:set_current(info)
  logger.debug(function() return 'ActivityManager.new: workspace=' .. info.dir end)

  if not has_initialized then
    local err = initialize_global()
    if err then
      logger.notify(err, vim.log.levels.ERROR)
      error('Failed to initialize ActivityManager', 0)
    end
    has_initialized = true
  end

  return self
end)

--------------------------------------------------------------------------------
-- Lifecycle
--------------------------------------------------------------------------------

function ActivityManager:run()
  logger.debug 'ActivityManager.run'
  self.is_ready = true
  self.idle_timer:record_activity()

  async.run(function()
    hooks.run('ready', self)
    hooks.run('buf_enter', self)
    self:queue_update(true)
  end)

  if config.advanced.plugin.autocmds then self.autocmds.setup() end
  self.idle_timer:start()
end

function ActivityManager:cleanup()
  logger.debug 'ActivityManager.cleanup'
  self.is_ready = false
  self.autocmds.clear()
  self.debouncer:cleanup()
  self.idle_timer:cleanup()
end

--------------------------------------------------------------------------------
-- Activity Updates
--------------------------------------------------------------------------------

---@param force? boolean
function ActivityManager:queue_update(force)
  if not self.events_enabled or not self.is_ready then return end

  self.debouncer:request(force, function(debounced_force)
    async.run(function() self:do_update(debounced_force) end)
  end)
end

---@param force? boolean
function ActivityManager:do_update(force)
  if not self.events_enabled or not self.is_ready then return end

  self.opts = self.options_builder:build()

  if not has_loaded_workspace then
    has_loaded_workspace = true
    hooks.run('workspace_change', self.opts)
    if self.is_paused then return end
  end

  local should_update = force or OptionsBuilder.has_changed(self.opts, self.last_opts)
  if not self.idle_timer.is_forced and should_update then
    logger.debug(
      function() return 'ActivityManager.do_update: updating; force=' .. tostring(force) end
    )
    if self.idle_timer.is_idle then hooks.run('idle_leave', self.opts) end
    self.activity_updater:update()
  end
end

--------------------------------------------------------------------------------
-- Public API
--------------------------------------------------------------------------------

---@param activity table
---@param force? boolean
function ActivityManager:set_activity(activity, force)
  if not self.is_ready or self.is_paused then return end

  hooks.run('post_activity', self.opts, activity)

  if config.timestamp.shared then
    self.opts.timestamp = nil
    if self.last_opts then self.last_opts.timestamp = nil end
  end

  if not force and self.should_skip_update then
    self.should_skip_update = false
    return
  end

  self.tx:update_activity(activity)
end

---@param force? boolean
function ActivityManager:clear_activity(force) self.tx:clear_activity(force) end

function ActivityManager:skip_update() self.should_skip_update = true end

function ActivityManager:idle()
  async.run(function() self.activity_updater:update_idle() end)
end

function ActivityManager:force_idle() self.idle_timer:force() end

function ActivityManager:unidle()
  self.idle_timer:leave()
  self:queue_update(true)
  logger.trace 'ActivityManager.unidle'
end

---@param force? boolean
function ActivityManager:toggle_idle(force)
  if self.idle_timer.is_forced or self.idle_timer.is_idle then
    self:unidle()
  elseif force then
    self:force_idle()
  else
    self:idle()
  end
end

function ActivityManager:pause()
  if self.is_paused then return end
  self.is_paused = true
  self:pause_events()
  self.idle_timer:stop()
  logger.debug 'ActivityManager.pause'
end

function ActivityManager:resume()
  if not self.is_paused then return end
  self.is_paused = false
  self:resume_events()
  self.idle_timer:reset()
  logger.debug 'ActivityManager.resume'
end

function ActivityManager:pause_events() self.events_enabled = false end

function ActivityManager:resume_events()
  self.events_enabled = true
  self:queue_update(true)
  logger.trace 'ActivityManager.resume_events'
end

function ActivityManager:hide()
  self:pause()
  self:clear_activity(true)
  logger.debug 'ActivityManager.hide'
end

function ActivityManager:suppress()
  self:pause()
  self:clear_activity()
  logger.debug 'ActivityManager.suppress'
end

function ActivityManager:toggle()
  if self.is_paused then
    self:resume()
  else
    self:hide()
  end
end

function ActivityManager:toggle_suppress()
  if self.is_paused then
    self:resume()
  else
    self:suppress()
  end
end

--------------------------------------------------------------------------------
-- Event handlers (delegate to EventHandler)
--------------------------------------------------------------------------------

function ActivityManager:on_buf_enter() self.event_handler:on_buf_enter() end

function ActivityManager:on_focus_gained() self.event_handler:on_focus_gained() end

function ActivityManager:on_focus_lost() self.event_handler:on_focus_lost() end

function ActivityManager:on_cursor_update() self.event_handler:on_cursor_update() end

return ActivityManager
