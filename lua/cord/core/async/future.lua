---@class Future
---@field _state 'pending' | 'fulfilled' | 'rejected'
---@field _value any
---@field _callbacks { on_fulfilled: fun(value: any), on_rejected: fun(reason: any) }[]
local Future = {}
Future.__index = Future

function Future.new(executor)
  local self = setmetatable({
    _state = 'pending',
    _value = nil,
    _callbacks = {},
  }, Future)

  local function complete(state, value)
    if self._state ~= 'pending' then return end
    self._state = state
    self._value = value
    local cbs = self._callbacks
    for i = 1, #cbs do
      if state == 'fulfilled' then
        cbs[i].on_fulfilled(value)
      else
        cbs[i].on_rejected(value)
      end
    end
  end

  local ok, err = pcall(executor,
    function(v) complete('fulfilled', v) end,
    function(e) complete('rejected', e) end
  )

  if not ok then
    require('cord.api.log').trace(
      function() return 'Error in executor: ' .. tostring(err) .. '\n' .. debug.traceback() end
    )
    complete('rejected', err)
  end

  return self
end

local function chain_result(result, resolve, reject)
  if type(result) == 'table' and result._state then
    result:and_then(resolve, reject)
  else
    resolve(result)
  end
end

function Future:and_then(on_ok, on_err)
  if not coroutine.running() then
    require('cord.api.log').error(
      function() return 'Future:and_then must be called within a coroutine\n' .. debug.traceback() end
    )
    return
  end

  local parent = self
  return Future.new(function(resolve, reject)
    local function run_handler(handler, value, passthrough)
      if type(handler) ~= 'function' then
        passthrough(value)
        return
      end
      local ok, res = pcall(handler, value)
      if ok then
        chain_result(res, resolve, reject)
      else
        require('cord.api.log').trace(
          function() return 'Error in handler: ' .. tostring(res) .. '\n' .. debug.traceback() end
        )
        reject(res)
      end
    end

    if parent._state == 'pending' then
      parent._callbacks[#parent._callbacks + 1] = {
        on_fulfilled = function(v) run_handler(on_ok, v, resolve) end,
        on_rejected = function(e) run_handler(on_err, e, reject) end,
      }
    elseif parent._state == 'fulfilled' then
      vim.schedule(function() run_handler(on_ok, parent._value, resolve) end)
    else
      vim.schedule(function() run_handler(on_err, parent._value, reject) end)
    end
  end)
end

function Future:catch(handler)
  return self:and_then(nil, handler)
end

function Future.await(f)
  if not coroutine.running() then
    require('cord.api.log').error(
      function() return 'Future.await must be called within a coroutine\n' .. debug.traceback() end
    )
    return
  end
  local ok, res = coroutine.yield(f)
  if ok then return res else error(res, 0) end
end

function Future.get(f)
  if not coroutine.running() then
    require('cord.api.log').error(
      function() return 'Future.get must be called within a coroutine\n' .. debug.traceback() end
    )
    return
  end
  local ok, res = coroutine.yield(f)
  if ok then return res else return nil, res end
end

function Future.all(list)
  return Future.new(function(resolve, reject)
    local count = #list
    if count == 0 then
      resolve({})
      return
    end

    local out, done, failed = {}, 0, false

    for idx, f in ipairs(list) do
      if f._state == 'fulfilled' then
        out[idx] = f._value
        done = done + 1
      elseif f._state == 'rejected' then
        if not failed then
          failed = true
          reject(f._value)
        end
      else
        f._callbacks[#f._callbacks + 1] = {
          on_fulfilled = function(v)
            out[idx] = v
            done = done + 1
            if done == count and not failed then resolve(out) end
          end,
          on_rejected = function(e)
            if not failed then
              failed = true
              reject(e)
            end
          end,
        }
      end
    end

    if done == count and not failed then resolve(out) end
  end)
end

return Future
