---@diagnostic disable-next-line: deprecated
local unpack = unpack or table.unpack
local logger = require 'cord.api.log'

local M = {}

local hooks = {
  ready = {},
  shutdown = {},

  pre_activity = {},
  post_activity = {},

  idle_enter = {},
  idle_leave = {},

  workspace_change = {},
}

---Constants for common priority levels
M.PRIORITY = {
  HIGHEST = 100,
  HIGH = 75,
  NORMAL = 50,
  LOW = 25,
  LOWEST = 0,
}

---Register a hook function with priority
---@param event string Event name to hook into
---@param fn function Function to call
---@param priority? number Priority (0-100, default NORMAL)
function M.register(event, fn, priority)
  local hook = hooks[event]
  if not hook then
    error(string.format('Invalid hook event: %s', event))
    return
  end

  priority = priority or M.PRIORITY.NORMAL

  table.insert(hook, {
    fn = fn,
    priority = priority,
  })

  -- Sort by priority (higher first)
  table.sort(hook, function(a, b) return a.priority > b.priority end)
end

---Run all hooks for an event
---@param event string Event name
---@param ... any Arguments to pass to hooks
function M.run(event, ...)
  logger.trace(function() return 'Hooks.run: event=' .. tostring(event) end)

  local hook = hooks[event]
  if not hook then return end

  local args = { ... }
  for _, ihook in ipairs(hook) do
    ihook.fn(unpack(args))
  end
end

return M
