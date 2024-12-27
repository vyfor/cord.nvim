local async = require 'cord.core.async'
local activities = require 'cord.plugin.activity'
local ws_utils = require 'cord.plugin.fs.workspace'
local config_utils = require 'cord.plugin.config.util'

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
---@field details? string Detailed information about what the user is doing
---@field state? string Secondary information about what the user is doing
---@field timestamps? ActivityTimestamps Contains `start` and `end` timestamps for the activity
---@field assets? ActivityAssets Defines images and tooltips, including `large_image`, `large_text`, `small_image`, and `small_text`
---@field buttons? CordButtonConfig[] Array of objects, each with `label` and `url`, defining interactive buttons in the presence
---@field is_idle? boolean Whether the activity should be considered as idle

---@class ActivityTimestamps
---@field start? integer Start timestamp in milliseconds
---@field end? integer End timestamp in milliseconds

---@class ActivityAssets
---@field large_image? string Large image ID or URL
---@field large_text? string Large image text
---@field small_image? string Small image ID or URL
---@field small_text? string Small image text

---@class CordOpts
---@field manager ActivityManager Reference to the ActivityManager instance
---@field filename string Current buffer's filename
---@field filetype string Current buffer's filetype
---@field is_read_only boolean Whether the current buffer is read-only
---@field cursor_line integer Current cursor line
---@field cursor_char integer Current cursor character
---@field timestamp number Timestamp passed to the Rich Presence in milliseconds
---@field workspace_dir string Current workspace directory
---@field workspace_name string Current workspace name
---@field repo_url? string Current Git repository URL, if any
---@field is_focused boolean Whether Neovim is focused
---@field is_idle boolean Whether the session is idle
---@field buttons CordButtonConfig[] Buttons configuration
---@field type string Which category the asset belongs to, e.g. 'language' or 'docs'
---@field name? string Asset name, if any
---@field icon string Asset icon URL or name, if any
---@field tooltip string Asset tooltip text, if any
---@field text string Asset text, if any

---Create a new ActivityManager instance
---@param opts {config: CordConfig, tx: table} Configuration and transmitter options
ActivityManager.new = async.wrap(function(opts)
  local self = setmetatable({
    config = opts.config,
    tx = opts.tx,
    is_focused = true,
    is_paused = false,
    is_force_idle = false,
    events_enabled = true,
    workspace_cache = {},
  }, mt)

  local rawdir = vim.fn.expand '%:p:h'
  local dir = ws_utils.find(rawdir):get() or vim.fn.getcwd()
  self.workspace_dir = dir

  local cache = {
    dir = dir,
    name = vim.fn.fnamemodify(dir, ':t'),
  }

  local repo_url = ws_utils.find_git_repository(dir):get()
  if repo_url then
    self.repo_url = repo_url
    cache.repo_url = repo_url
  end

  self.workspace_cache[rawdir] = cache

  return self
end)

---Run the activity manager
---@return nil
function ActivityManager:run()
  self.workspace_name = vim.fn.fnamemodify(self.workspace_dir, ':t')
  self.last_updated = uv.now()

  if self.config.timestamp.enabled then self.timestamp = os.time() end
  if self.config.hooks.on_ready then self.config.hooks.on_ready(self) end

  self:queue_update(true)
  if self.config.advanced.plugin.autocmds then self:setup_autocmds() end

  if self.config.idle.enabled then
    self.idle_timer = uv.new_timer()
    self.idle_timer:start(
      0,
      self.config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end
end

---Clean up the activity manager
---@return nil
function ActivityManager:cleanup()
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

  return should_update
end

---Queue an activity update
---@param force_update? boolean Whether to force the update regardless of conditions
function ActivityManager:queue_update(force_update)
  if not self.events_enabled then return end

  self.opts = self:build_opts()
  if not self.is_force_idle and (force_update or self:should_update()) then
    self:update_activity()
  end
end

---Check if the activity should be updated to idle
---@return nil
function ActivityManager:check_idle()
  if not self.events_enabled then return end
  if not self.config.idle.enabled and not self.is_force_idle then return end
  if self.is_idle then return end

  local time_elapsed = uv.now() - self.last_updated
  if
    self.is_force_idle
    or (
      time_elapsed >= self.config.idle.timeout
      and (self.config.idle.ignore_focus or not self.is_focused)
    )
  then
    self:update_idle_activity()
  else
    local time_remaining = self.config.idle.timeout - time_elapsed
    self.idle_timer:stop()
    self.idle_timer:start(
      time_remaining,
      0,
      vim.schedule_wrap(function()
        self.idle_timer:start(
          0,
          self.config.idle.timeout,
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

  if self.config.idle.show_status then
    local buttons = config_utils:get_buttons(self.opts)
    self.opts.buttons = buttons

    if self.config.hooks.on_update then
      self.config.hooks.on_update(self.opts)
    end

    local activity = activities.build_idle_activity(self.config, self.opts)

    if self.config.hooks.on_idle then
      self.config.hooks.on_idle(self.opts, activity)
    end

    if self.should_skip_update then
      self.should_skip_update = false
      return
    end

    self.tx:update_activity(activity)
  else
    if self.config.hooks.on_idle then self.config.hooks.on_idle(self.opts) end

    if self.should_skip_update then
      self.should_skip_update = false
      return
    end

    self.tx:clear_activity()
  end
end

---Update the activity
---@return nil
function ActivityManager:update_activity()
  if self:should_update_time() then self.timestamp = os.time() end

  self.is_idle = false
  self.is_force_idle = false
  self.last_opts = self.opts
  self.last_updated = uv.now()
  self.opts.is_idle = false

  if self.config.hooks.on_update then self.config.hooks.on_update(self.opts) end

  local activity = activities.build_activity(self.config, self.opts)

  if self.config.hooks.on_activity then
    self.config.hooks.on_activity(self.opts, activity)
  end

  if self.should_skip_update then
    self.should_skip_update = false
    return
  end

  self.tx:update_activity(activity)
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
      self.config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end
  self.is_paused = false
end

---Pause only events, keeping the current activity state
---@return nil
function ActivityManager:pause_events() self.events_enabled = false end

---Resume events after they were paused
---@return nil
function ActivityManager:resume_events()
  self.events_enabled = true
  self:queue_update(true)
end

---Force idle state
---@return nil
function ActivityManager:force_idle()
  self.is_force_idle = true
  self:update_idle_activity()
end

---Unforce idle state
---@return nil
function ActivityManager:unforce_idle()
  self.is_force_idle = false
  self:queue_update(true)
end

---Toggle idle state
---@return nil
function ActivityManager:toggle_idle()
  if self.is_force_idle then
    self:unforce_idle()
  else
    self:force_idle()
  end
end

---Hide the activity
---@return nil
function ActivityManager:hide()
  self:pause()
  self:clear_activity()
end

---Toggle the activity
---@return nil
function ActivityManager:toggle()
  if self.is_paused then
    self:resume()
  else
    self:hide()
  end
end

---Setup autocmds
---@return nil
function ActivityManager:setup_autocmds()
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
      autocmd BufEnter * lua require'cord.server'.manager:on_buf_enter()
      autocmd FocusGained * lua require'cord.server'.manager:on_focus_gained()
      autocmd FocusLost * lua require'cord.server'.manager:on_focus_lost()
    augroup END
  ]]

  if self.config.advanced.cursor_update_mode == 'on_hold' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorHold,CursorHoldI * lua require'cord.server'.manager:on_cursor_update()
      augroup END
    ]]
  elseif self.config.advanced.cursor_update_mode == 'on_move' then
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
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
    augroup END
  ]]
end

---Set the activity
---@param activity table Activity to set
---@return nil
function ActivityManager:set_activity(activity)
  self.tx:update_activity(activity)
end

---Clear the activity
---@param force? boolean Whether to force clear the activity
---@return nil
function ActivityManager:clear_activity(force) self.tx:clear_activity(force) end

---Check if the time should be updated
---@return boolean Whether the time should be updated
function ActivityManager:should_update_time()
  return self.config.timestamp.enabled
      and (self.config.timestamp.reset_on_change or self.config.timestamp.reset_on_idle and self.is_idle)
    or false
end

---Handle buffer enter event
---@return nil
function ActivityManager:on_buf_enter()
  local rawdir = vim.fn.expand '%:p:h'
  local cached = self.workspace_cache[rawdir]
  if cached then
    if cached.dir ~= self.workspace_dir then
      self.workspace_dir = cached.dir
      self.workspace_name = cached.name
      self.repo_url = cached.repo_url
      self.opts.workspace_dir = self.workspace_dir
      self.opts.workspace_name = self.workspace_name
      self.opts.repo_url = self.repo_url

      if self.config.hooks.on_workspace_change then
        self.config.hooks.on_workspace_change(self.opts)
      end
    end

    self:queue_update()
    return
  elseif cached == false then
    if self.workspace_dir then
      self.workspace_dir = nil
      self.workspace_name = nil
      self.repo_url = nil
      self.opts.workspace_dir = nil
      self.opts.workspace_name = nil
      self.opts.repo_url = nil

      if self.config.hooks.on_workspace_change then
        self.config.hooks.on_workspace_change(self.opts)
      end
    end

    self:queue_update()
    return
  end

  async.run(function()
    local ws_utils = require 'cord.plugin.fs.workspace'
    local dir = ws_utils.find(vim.fn.expand '%:p:h'):get() or vim.fn.getcwd()

    if not dir then
      self.workspace_cache[rawdir] = false
      self:queue_update()
      return
    end

    self.workspace_dir = dir
    self.workspace_name = vim.fn.fnamemodify(self.workspace_dir, ':t')
    self.opts.workspace_dir = self.workspace_dir
    self.opts.workspace_name = self.workspace_name

    local repo_url = ws_utils.find_git_repository(self.workspace_dir):get()
    self.repo_url = repo_url
    self.opts.repo_url = repo_url

    self.workspace_cache[rawdir] = {
      dir = self.workspace_dir,
      name = self.workspace_name,
      repo_url = self.repo_url,
    }

    if self.config.hooks.on_workspace_change then
      self.config.hooks.on_workspace_change(self.opts)
    end

    self:queue_update()
  end)
end

---Handle focus gained event
---@return nil
function ActivityManager:on_focus_gained()
  if not self.events_enabled then return end
  self.is_focused = true
  self.opts.is_focused = true

  if self.config.idle.unidle_on_focus then self:queue_update(true) end
end

---Handle focus lost event
---@return nil
function ActivityManager:on_focus_lost()
  if not self.events_enabled then return end
  self.is_focused = false
  self.opts.is_focused = false
end

---Handle cursor update event
---@return nil
function ActivityManager:on_cursor_update()
  if not self.events_enabled then return end
  self:queue_update()
end

---Build options
---@return CordOpts
function ActivityManager:build_opts()
  local cursor_position = vim.api.nvim_win_get_cursor(0)
  local opts = {
    manager = self,
    filename = vim.fn.expand '%:t',
    filetype = vim.bo.filetype,
    is_read_only = vim.bo.readonly,
    cursor_line = cursor_position[1],
    cursor_char = cursor_position[2],
    timestamp = self.timestamp,
    workspace_dir = self.workspace_dir,
    workspace_name = self.workspace_name,
    repo_url = self.repo_url,
    is_focused = self.is_focused,
    is_idle = self.is_idle,
  }
  local buttons = config_utils:get_buttons(opts)
  opts.buttons = buttons

  return opts
end

return ActivityManager
