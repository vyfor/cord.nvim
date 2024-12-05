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
  self.is_paused = false
  self.is_force_idle = false
  self.last_activity = nil
  self.last_opts = nil

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

function ActivityManager:queue_update(force_update)
  vim.schedule(function() self:process_update(force_update) end)
end

function ActivityManager:process_update(force_update)
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

  if not self.is_force_idle and (force_update or self:should_update(opts)) then
    self:update_activity(opts)
  elseif not self.is_idle then
    self:check_idle(opts)
  end
end

function ActivityManager:should_update(opts)
  local should_update = not self.last_opts
    or opts.filename ~= self.last_opts.filename
    or opts.filetype ~= self.last_opts.filetype
    or opts.is_read_only ~= self.last_opts.is_read_only
    or opts.cursor_line ~= self.last_opts.cursor_line
    or opts.cursor_char ~= self.last_opts.cursor_char
    or opts.is_focused ~= self.last_opts.is_focused

  return should_update
end

function ActivityManager:run()
  self.last_updated = uv.now()
  if self.config.timestamp.enable then self.timestamp = os.time() end

  if self.config.idle.enable then
    self.idle_timer = uv.new_timer()
    self.idle_timer:start(
      0,
      self.config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end

  self:setup_autocmds()
  if self.config.usercmds then self:setup_usercmds() end
  self:queue_update(true)
end

function ActivityManager:check_idle(opts)
  if not self.config.idle.enable and not self.is_force_idle then return end
  if self.is_idle then return end

  local time_elapsed = uv.now() - self.last_updated
  if
    self.is_force_idle
    or (
      time_elapsed > self.config.idle.timeout
      and (self.config.idle.ignore_focus or not self.is_focused)
    )
  then
    self:update_idle_activity(opts or self.last_opts)
  end
end

function ActivityManager:update_idle_activity(opts)
  self.is_idle = true
  self.last_updated = uv.now()

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

function ActivityManager:update_activity(opts)
  if self:should_update_time() then self.timestamp = os.time() end

  self.is_idle = false
  self.is_force_idle = false
  self.last_opts = opts
  self.last_updated = uv.now()

  local activity = activities.build_activity(self.config, opts)
  if self.config.hooks.on_update then
    self.config.hooks.on_update(opts, activity)
  end

  self.tx:update_activity(activity)
end

function ActivityManager:pause()
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
    augroup END
  ]]
  if self.idle_timer then self.idle_timer:stop() end
  self.is_paused = true
end

function ActivityManager:resume()
  self:setup_autocmds()
  if self.idle_timer then
    self.idle_timer:start(
      0,
      self.config.idle.timeout,
      vim.schedule_wrap(function() self:check_idle() end)
    )
  end
  self.is_paused = false
end

function ActivityManager:hide()
  self:pause()
  self:clear_activity()
end

function ActivityManager:toggle()
  if self.is_paused then
    self:resume()
  else
    self:hide()
  end
end

function ActivityManager:force_idle()
  self.is_force_idle = true
  self:queue_update()
end

function ActivityManager:unforce_idle()
  self.is_force_idle = false
  self:queue_update(true)
end

function ActivityManager:toggle_idle()
  if self.is_force_idle then
    self:unforce_idle()
  else
    self:force_idle()
  end
end

function ActivityManager:setup_usercmds()
  if not self.config.usercmds then return end

  vim.cmd [[
    command! CordShowPresence lua require'cord'.manager:resume()
    command! CordHidePresence lua require'cord'.manager:hide()
    command! CordTogglePresence lua require'cord'.manager:toggle()
    command! CordIdle lua require'cord'.manager:force_idle()
    command! CordUnidle lua require'cord'.manager:unforce_idle()
    command! CordToggleIdle lua require'cord'.manager:toggle_idle()
    command! -bang CordClearPresence lua require'cord'.manager:clear_activity('<bang>' == '!')
  ]]
end

function ActivityManager:on_buf_enter()
  local new_workspace_dir = vim.fn.expand '%:p:h'
  if new_workspace_dir ~= self.workspace_dir then
    self.workspace_dir = new_workspace_dir
    self.workspace_name = ws_utils.find(self.workspace_dir)
    if self.config.hooks.on_workspace_change then
      local opts = self.last_opts
      opts.workspace_dir = self.workspace_dir
      opts.workspace_name = self.workspace_name
      self.config.hooks.on_workspace_change(opts)
    end
  end

  self:queue_update()
end

function ActivityManager:on_focus_gained()
  self.is_focused = true
  self:queue_update()
end

function ActivityManager:on_focus_lost()
  self.is_focused = false
  self:queue_update()
end

function ActivityManager:on_dir_changed()
  local new_workspace_dir = vim.fn.expand '%:p:h'
  if new_workspace_dir ~= self.workspace_dir then
    self.workspace_dir = new_workspace_dir
    self.workspace_name = ws_utils.find(self.workspace_dir)
    self:queue_update()
  end
end

function ActivityManager:on_cursor_update() self:queue_update() end

function ActivityManager:setup_autocmds()
  vim.cmd [[
    augroup CordActivityManager
      autocmd!
      autocmd BufEnter * lua require'cord'.manager:on_buf_enter()
      autocmd FocusGained * lua require'cord'.manager:on_focus_gained()
      autocmd FocusLost * lua require'cord'.manager:on_focus_lost()
      autocmd DirChanged * lua require'cord'.manager:on_dir_changed()
    augroup END
  ]]

  if self.config.advanced.cursor_update_mode == 'on_hold' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorHold,CursorHoldI * lua require'cord'.manager:on_cursor_update()
      augroup END
    ]]
  elseif self.config.advanced.cursor_update_mode == 'on_move' then
    vim.cmd [[
      augroup CordActivityManager
        autocmd CursorMoved,CursorMovedI * lua require'cord'.manager:on_cursor_update()
      augroup END
    ]]
  end

  self:queue_update(true)
end

function ActivityManager:clear_activity(force) self.tx:clear_activity(force) end

function ActivityManager:should_update_time()
  return self.config.display.show_time
    and (
      self.config.timestamp.reset_on_change
      or self.config.timestamp.reset_on_idle and self.is_idle
    )
end

return ActivityManager
