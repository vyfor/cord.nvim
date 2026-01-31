local Future = require 'cord.core.async.future'

---@diagnostic disable-next-line: deprecated
local unpack = unpack or table.unpack

---@class Async
local Async = {}

---Callable that returns a Future when invoked.
---@class AsyncFunction
---@field _fn fun(...): Future
---@overload fun(...): Future

---@type AsyncFunction|metatable
local async_mt = { __name = 'async_function' }
async_mt.__call = function(self, ...)
  ---@cast self AsyncFunction
  return self[1](...)
end

local getmetatable = getmetatable

---Checks if a value is an async-wrapped function.
---@param fn any
---@return boolean
function Async.is_async(fn)
  return getmetatable(fn) == async_mt
end

---Wraps a function to return a Future when called within a coroutine.
---@param fn function The function to wrap
---@return AsyncFunction wrapped A callable that returns a Future
function Async.wrap(fn)
  ---@diagnostic disable-next-line: missing-fields
  return setmetatable({
    function(...)
      local args, n = { ... }, select('#', ...)
      return Future.new(function(resolve, reject)
        if not coroutine.running() then
          require('cord.api.log').error(
            function() return 'async.wrap must be called within a coroutine\n' .. debug.traceback() end
          )
          return
        end

        local ok, result = pcall(fn, unpack(args, 1, n))
        if not ok then
          require('cord.api.log').trace(
            function() return 'Error in async.wrap: ' .. tostring(result) .. '\n' .. debug.traceback() end
          )
          reject(result)
          return
        end

        if type(result) == 'table' and result._state then
          result:next(resolve, reject)
        else
          resolve(result)
        end
      end)
    end,
  }, async_mt)
end

---@param co thread
---@param ok boolean
---@param val any
local function drive_coroutine(co, ok, val)
  while coroutine.status(co) ~= 'dead' do
    local success, yielded = coroutine.resume(co, ok, val)
    if not success then error(debug.traceback(co, yielded), 0) end
    if coroutine.status(co) == 'dead' then break end

    if type(yielded) ~= 'table' or not yielded._state then
      ok, val = true, yielded
    elseif yielded._state ~= 'pending' then
      ok, val = yielded._state == 'fulfilled', yielded._value
    else
      local done, done_ok, done_val
      yielded._callbacks[#yielded._callbacks + 1] = {
        on_fulfilled = function(v)
          if done == nil then
            done, done_ok, done_val = true, true, v
          else
            vim.schedule(function() drive_coroutine(co, true, v) end)
          end
        end,
        on_rejected = function(e)
          if done == nil then
            done, done_ok, done_val = true, false, e
          else
            vim.schedule(function() drive_coroutine(co, false, e) end)
          end
        end,
      }
      done = false
      if done_ok ~= nil then
        ok, val = done_ok, done_val
      else
        return
      end
    end
  end
end

---Runs a function as a managed coroutine.
---@param fn function
---@return thread
function Async.run(fn)
  local co = coroutine.create(fn)
  drive_coroutine(co, true, nil)
  return co
end

return Async
