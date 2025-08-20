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

return M
