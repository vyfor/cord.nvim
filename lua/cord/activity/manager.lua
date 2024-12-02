local activities = require 'cord.activity'
local ws_utils = require 'cord.util.workspace'
local config_utils = require 'cord.util.config'

local uv = vim.loop

local ActivityManager = {}
local mt = { __index = ActivityManager }

function ActivityManager.new(opts)
  local self = setmetatable({}, mt)

  self.config = opts.config
  self.tx = opts.tx
  self.workspace_dir = ws_utils.find(vim.fn.expand '%:p:h')
  self.workspace_name = vim.fn.fnamemodify(self.workspace_dir, ':t')
  self.is_focused = true

  if config_utils:contains_git_url() then
    self.git_url = ws_utils.find_git_repository(self.workspace_dir)

    if self.config.buttons then
      for i = 1, #self.config.buttons do
        if self.config.buttons[i].url == 'git' then
          self.config.buttons[i].url = self.git_url
        end
      end
    end
  end

  return self
end

function ActivityManager:run_event_loop()
  self.last_updated = uv.now()
  if self.config.timestamp.enable then self.timestamp = os.time() end

  self.timer = uv.new_timer()
  self.timer:start(
    0,
    self.config.advanced.interval,
    vim.schedule_wrap(function() self:on_tick() end)
  )
end

function ActivityManager:on_tick()
  local cursor_position = vim.api.nvim_win_get_cursor(0)
  local buttons = config_utils:get_buttons()

  local opts = {
    manager = self,
    filename = vim.fn.expand '%:t',
    filetype = vim.bo.filetype,
    is_read_only = vim.bo.readonly,
    cursor_line = cursor_position[1],
    cursor_char = cursor_position[2],
    timestamp = self.timestamp,
    buttons = buttons,
    workspace_dir = self.workspace_dir,
    workspace_name = self.workspace_name,
    git_url = self.git_url,
    is_focused = self.is_focused,
    is_idle = self.is_idle,
  }

  if self:should_update(opts) then
    self:update_activity(opts)
  else
    self:update_idle_activity(opts)
  end
end

function ActivityManager:update_activity(opts)
  if self:should_update_time() then self.timestamp = os.time() end

  self.is_idle = false
  self.force_idle = false
  self.last_opts = opts
  self.last_updated = uv.now()

  local activity = activities.build_activity(self.config, opts)
  if self.config.hooks.on_update then
    self.config.hooks.on_update(opts, activity)
  end

  self.tx:update_activity(activity)
end

function ActivityManager:update_idle_activity(opts)
  if self.is_idle then return end

  if
    self.force_idle
    or self.config.idle.enable
      and uv.now() - self.last_updated > self.config.idle.timeout
      and (self.config.idle.ignore_focus or not self.is_focused)
  then
    self.is_idle = true

    if self.config.idle.show_status then
      local activity = activities.build_idle_activity(self.config, opts)
      if self.config.hooks.on_idle then
        self.config.hooks.on_idle(opts, activity)
      end

      self.tx:update_activity(activity)
    else
      if self.config.hooks.on_idle then self.config.hooks.on_idle(opts) end

      self.tx:clear_activity()
    end
  end
end

function ActivityManager:clear_activity()
  self.last_opts = nil
  self.last_updated = uv.now()
  self.tx:clear_activity()
end

function ActivityManager:should_update(opts)
  return not self.last_opts
    or opts.filename ~= self.last_opts.filename
    or opts.filetype ~= self.last_opts.filetype
    or opts.is_read_only ~= self.last_opts.is_read_only
    or opts.cursor_line ~= self.last_opts.cursor_line
    or opts.cursor_char ~= self.last_opts.cursor_char
end

function ActivityManager:should_update_time()
  return self.config.display.show_time
    and (
      self.config.timer.reset_on_change
      or self.config.timer.reset_on_idle and self.is_idle
    )
end

function ActivityManager:pause()
  if self.timer then self.timer:stop() end
end

function ActivityManager:resume()
  if self.timer then
    self.timer:start(
      0,
      self.config.advanced.interval,
      vim.schedule_wrap(function() self:on_tick() end)
    )
  end
end

function ActivityManager:setup_autocmds()
  function ActivityManager:on_focus_gained() self.is_focused = true end
  function ActivityManager:on_focus_lost() self.is_focused = false end
  function ActivityManager:on_dir_changed()
    local new_workspace_dir = vim.fn.expand '%:p:h'
    if new_workspace_dir ~= self.workspace_dir then
      self.workspace_dir = new_workspace_dir
      self.workspace_name = ws_utils.find(self.workspace_dir)
    end
  end

  vim.cmd [[
    autocmd! FocusGained * lua self:on_focus_gained()
    autocmd! FocusLost * lua self:on_focus_lost()
    autocmd! BufReadPost * lua self:on_dir_changed()
    autocmd! DirChanged * lua self:on_dir_changed()
  ]]
end

return ActivityManager
