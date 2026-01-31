---@diagnostic disable-next-line: deprecated
local unpack = unpack or table.unpack
local utils = require 'cord.core.util'

local M = {}

M.validate_type = function(value, allowed_types)
  for _, t in ipairs(allowed_types) do
    local ty = type(value)
    if t == 'table' and ty == 'table' then
      return true
    elseif t == 'string' and ty == 'string' then
      return true
    elseif t == 'number' and ty == 'number' then
      return true
    elseif t == 'boolean' and ty == 'boolean' then
      return true
    elseif t == 'function' and ty == 'function' then
      return true
    end
  end
  return false
end

M.is_valid_path = function(validation_rules, dict_paths, path)
  if validation_rules[path] then return true end

  local parts = vim.split(path, '.', { plain = true })
  if #parts >= 2 then
    local parent = parts[1]
    if dict_paths[parent] then return true end

    local wildcard_path = table.concat(
      utils.tbl_flatten {
        { parts[1] },
        { '*' },
        { unpack(parts, 3) },
      },
      '.'
    )
    return validation_rules[wildcard_path] ~= nil
  end
  return false
end

M.get_nested_value = function(config, path)
  local parts = vim.split(path, '.', { plain = true })
  local current = config
  for _, part in ipairs(parts) do
    if type(current) ~= 'table' then return nil end
    current = current[part]
  end
  return current
end

-- Source: https://github.com/neovim/neovim/blob/1e6c4ea896b784754cb0ba18ea510a9c407ab54c/runtime/lua/vim/_core/shared.lua#L535C1-L647C4
-- Modified to preserve metatables

local function can_merge(v)
  if type(v) ~= 'table' then
    return false
  end
  local mt = getmetatable(v)
  if mt and (vim._empty_dict_mt == nil or mt ~= vim._empty_dict_mt) then
    return false
  end
  return vim.tbl_isempty(v) or not vim.islist(v)
end

local function tbl_extend_rec(behavior, deep_extend, ...)
  local ret = {}
  if vim._empty_dict_mt ~= nil and getmetatable(select(1, ...)) == vim._empty_dict_mt then
    ret = vim.empty_dict()
  end

  for i = 1, select('#', ...) do
    local tbl = select(i, ...)
    if tbl then
      for k, v in pairs(tbl) do
        if deep_extend and can_merge(v) and can_merge(ret[k]) then
          ret[k] = tbl_extend_rec(behavior, true, ret[k], v)
        elseif behavior == 'force' or ret[k] == nil then
          ret[k] = v
        end
      end
    end
  end

  return ret
end

local function tbl_extend(behavior, deep_extend, ...)
  if behavior ~= 'keep' and behavior ~= 'force' then
    error('invalid "behavior": ' .. tostring(behavior))
  end

  local nargs = select('#', ...)

  if nargs < 2 then
    error(('wrong number of arguments (given %d, expected at least 3)'):format(1 + nargs))
  end

  return tbl_extend_rec(behavior, deep_extend, ...)
end

M.tbl_extend = function(behavior, ...)
  return tbl_extend(behavior, false, ...)
end

M.tbl_deep_extend = function(behavior, ...)
  return tbl_extend(behavior, true, ...)
end

return M
