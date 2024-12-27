local Future = require 'cord.core.async.future'
local uv = vim.loop or vim.uv

local IPC = {}
local mt = { __index = IPC }

function IPC.new()
  local self = setmetatable({}, mt)
  return self
end

---@async
function IPC:connect(path)
  return Future.new(function(resolve, reject)
    local pipe = uv.new_pipe()
    self.pipe = pipe

    pipe:connect(path, function(err)
      if err then
        pipe:close()
        reject(err)
        return
      end
      resolve()
    end)
  end)
end

function IPC:read_start(callback)
  self.pipe:read_start(function(err, chunk)
    if err or not chunk then
      self:close()
      return
    end

    callback(chunk)
  end)
end

---@async
function IPC:read()
  return Future.new(function(resolve, reject)
    if not self.pipe then
      reject 'No pipe connection'
      return
    end

    self.pipe:read_start(function(err, chunk)
      if err then
        self:close()
        reject(err)
        return
      end

      if chunk then
        resolve(chunk)
      else
        self:close()
        resolve(nil)
      end
    end)
  end)
end

---@async
function IPC:write(data)
  return Future.new(function(resolve, reject)
    if not self.pipe then
      reject 'No pipe connection'
      return
    end

    self.pipe:write(data, function(err)
      if err then
        self:close()
        reject(err)
        return
      end
      resolve(true)
    end)
  end)
end

function IPC:close()
  if self.pipe then
    self.pipe:read_stop()
    if not self.pipe:is_closing() then self.pipe:close() end
    self.pipe = nil
  end

  if self.on_close then self.on_close() end
end

function IPC:is_closing() return not self.pipe or self.pipe:is_closing() end

return IPC
