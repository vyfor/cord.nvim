---@class Future
---@field _state 'pending' | 'fulfilled' | 'rejected'
---@field _value any
---@field _callbacks { on_fulfilled: fun(value: any), on_rejected: fun(reason: any) }[]
local Future = {}
local mt = { __index = Future }

function Future.new(executor)
  local self = setmetatable({}, mt)
  self._state = 'pending'
  self._value = nil
  self._callbacks = {}

  local function resolve(value)
    if self._state ~= 'pending' then return end
    self._state = 'fulfilled'
    self._value = value
    for _, callback in ipairs(self._callbacks) do
      callback.on_fulfilled(value)
    end
  end

  local function reject(reason)
    if self._state ~= 'pending' then return end
    self._state = 'rejected'
    self._value = reason
    for _, callback in ipairs(self._callbacks) do
      callback.on_rejected(reason)
    end
  end

  xpcall(function() executor(resolve, reject) end, function(err)
    require('cord.plugin.log').tracecb(
      function()
        return 'Error in executor: ' .. err .. '\n' .. debug.traceback()
      end
    )
    reject(err)
  end)

  return self
end

function Future:and_then(on_fulfilled, on_rejected)
  local current = coroutine.running()
  if not current then
    require('cord.plugin.log').errorcb(
      function()
        return 'Future:and_then must be called within a coroutine\n'
          .. debug.traceback()
      end
    )
    return
  end

  return Future.new(function(resolve, reject)
    local function handle_callback(callback, resolve, reject, value)
      if type(callback) ~= 'function' then
        if self._state == 'fulfilled' then
          resolve(value or self._value)
        else
          reject(value or self._value)
        end
        return
      end

      local success, result = xpcall(
        function() return callback(value or self._value) end,
        function(err)
          require('cord.plugin.log').tracecb(
            function()
              return 'Error in callback: ' .. err .. '\n' .. debug.traceback()
            end
          )
        end
      )

      if not success then
        reject(result)
        return
      end

      if type(result) == 'table' and result._state then
        result:and_then(resolve, reject)
      else
        resolve(result)
      end
    end

    if self._state == 'pending' then
      table.insert(self._callbacks, {
        on_fulfilled = function(value)
          handle_callback(on_fulfilled, resolve, reject, value)
        end,
        on_rejected = function(reason)
          handle_callback(on_rejected, resolve, reject, reason)
        end,
      })
    else
      vim.defer_fn(function()
        if self._state == 'fulfilled' then
          handle_callback(on_fulfilled, resolve, reject)
        else
          handle_callback(on_rejected, resolve, reject)
        end
      end, 0)
    end
  end)
end

function Future:catch(on_rejected) return self:and_then(nil, on_rejected) end

function Future.await(future)
  local co = coroutine.running()
  if not co then
    require('cord.plugin.log').errorcb(
      function()
        return 'Future:await must be called within a coroutine\n'
          .. debug.traceback()
      end
    )
    return
  end

  future:and_then(
    function(value) coroutine.resume(co, true, value) end,
    function(reason) coroutine.resume(co, false, reason) end
  )

  local success, result = coroutine.yield()
  if success then
    return result
  else
    error(result, 0)
  end
end

function Future.get(future)
  local co = coroutine.running()
  if not co then
    require('cord.plugin.log').errorcb(
      function()
        return 'Future:get must be called within a coroutine\n'
          .. debug.traceback()
      end
    )
    return
  end

  future:and_then(
    function(value) coroutine.resume(co, true, value) end,
    function(reason) coroutine.resume(co, false, reason) end
  )

  local success, result = coroutine.yield()
  if success then
    return result
  else
    return nil, result
  end
end

function Future.all(futures)
  return Future.new(function(resolve, reject)
    local results = {}
    local completed = 0
    for i, future in ipairs(futures) do
      future
        :and_then(function(result)
          results[i] = result
          completed = completed + 1
          if completed == #futures then resolve(results) end
        end)
        :catch(reject)
    end
  end)
end

return Future
