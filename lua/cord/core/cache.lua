local M = {}

---@class CacheEntry
---@field value any
---@field expiry integer|nil

---@class CordCache
---@field _data table<any, CacheEntry>
local Cache = {}
Cache.__index = Cache

---Create a new cache instance
---@return CordCache
function M.new() return setmetatable({ _data = {} }, Cache) end

---Get a value from the cache
---@param key any
---@return any|nil
function Cache:get(key)
  local entry = self._data[key]
  if not entry then return nil end

  if entry.expiry and os.time() > entry.expiry then
    self._data[key] = nil
    return nil
  end

  return entry.value
end

---Set a value in the cache
---@param key any
---@param value any
---@param ttl? integer Time to live in seconds
function Cache:set(key, value, ttl)
  self._data[key] = {
    value = value,
    expiry = ttl and (os.time() + ttl) or nil,
  }
end

---Clear the cache
function Cache:clear() self._data = {} end

---Get a value or compute it if missing or expired.
---@param key any
---@param ttl integer|nil Time to live in seconds
---@param computer fun(): any
---@return any
function Cache:get_or_compute(key, ttl, computer)
  local val = self:get(key)
  if val ~= nil then return val end

  val = computer()

  self:set(key, val, ttl)
  return val
end

return M
