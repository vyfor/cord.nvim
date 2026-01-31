local Future = require 'cord.core.async.future'

local Async = {}

---@diagnostic disable-next-line: deprecated
local unpack_args = unpack or table.unpack

function Async.wrap(fn)
  return function(...)
    local args = { ... }
    return Future.new(function(resolve, reject)
      if not coroutine.running() then
        require('cord.api.log').error(
          function() return 'async.wrap must be called within a coroutine\n' .. debug.traceback() end
        )
        return
      end

      local ok, result = pcall(fn, unpack_args(args))
      if not ok then
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

local function drive_coroutine(co, ok, val)
  while coroutine.status(co) ~= 'dead' do
    local res = { coroutine.resume(co, ok, val) }
    if not res[1] then error(res[2], 0) end
    if coroutine.status(co) == 'dead' then break end

    local yielded = res[2]
    if type(yielded) ~= 'table' or not yielded._state then
      ok, val = true, yielded
    elseif yielded._state ~= 'pending' then
      ok = yielded._state == 'fulfilled'
      val = yielded._value
    else
      local resolved_inline, inline_ok, inline_val
      yielded._callbacks[#yielded._callbacks + 1] = {
        on_fulfilled = function(v)
          if resolved_inline == nil then
            resolved_inline, inline_ok, inline_val = true, true, v
          else
            vim.schedule(function() drive_coroutine(co, true, v) end)
          end
        end,
        on_rejected = function(e)
          if resolved_inline == nil then
            resolved_inline, inline_ok, inline_val = true, false, e
          else
            vim.schedule(function() drive_coroutine(co, false, e) end)
          end
        end,
      }
      resolved_inline = false
      if inline_ok ~= nil then
        ok, val = inline_ok, inline_val
      else
        return
      end
    end
  end
end

function Async.run(fn)
  local co = coroutine.create(fn)
  drive_coroutine(co, true, nil)
  return co
end

return Async
