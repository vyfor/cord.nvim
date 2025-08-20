local Future = require 'cord.core.async.future'

local Async = {}

function Async.wrap(fn)
  return function(...)
    local args = { ... }
    return Future.new(function(resolve, reject)
      local current = coroutine.running()
      if not current then
        require('cord.api.log').error(
          function() return 'async.wrap must be called within a coroutine\n' .. debug.traceback() end
        )
        return
      end

      local success, result = pcall(function()
        ---@diagnostic disable-next-line: deprecated
        local unpack = unpack or table.unpack
        return fn(unpack(args))
      end)
      if not success then
        require('cord.api.log').trace(
          function() return 'Error in async.wrap: ' .. result .. '\n' .. debug.traceback() end
        )
        reject(result)
        return
      end

      if type(result) == 'table' and result._state then
        result:and_then(resolve, reject)
      else
        resolve(result)
      end
    end)
  end
end

function Async.run(fn)
  local co = coroutine.create(fn)
  local function resume(success, ...)
    if not success then
      error(...)
      return
    end

    local ret = { coroutine.resume(co, ...) }
    success = table.remove(ret, 1)

    if success then
      if coroutine.status(co) ~= 'dead' then
        local future = ret[1]
        if future then
          if type(future) == 'table' and future._state then
            future:and_then(function(value)
              if coroutine.status(co) ~= 'dead' then resume(true, value) end
            end, function(err) resume(false, err) end)
          else
            resume(true, future)
          end
        end
      end
    else
      error(ret[1], 0)
    end
  end

  resume(true)
  return co
end

return Async
