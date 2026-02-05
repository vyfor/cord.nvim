local hooks = require 'cord.internal.hooks'
local config = require 'cord.api.config'
local config_util = require 'cord.api.config.util'
local logger = require 'cord.api.log'
local async = require 'cord.core.async'

local M = {}

M.HOOK_PRIORITY = hooks.PRIORITY

local extensions = {}
local variables = {}
local assets = {}
local configs = {}

---@class CordExtension
---@field name string Extension name
---@field description? string Extension description
---@field variables? table<string,function> Variables to add
---@field hooks? table<string,function>|{fun: function, priority: number} Hooks to register
---@field assets? table<string,string|CordAssetConfig> Assets to add
---@field config? table Configuration to merge

---Register an extension
---@param extension CordExtension Extension definition table
function M.register(extension)
  if not extension.name then
    error 'Extension must have a name'
    return
  end

  if extensions[extension.name] then return end

  extensions[extension.name] = extension

  if extension.variables then variables[extension.name] = extension.variables end

  if extension.assets then assets[extension.name] = extension.assets end

  if extension.hooks then
    for event, hook in pairs(extension.hooks) do
      if type(hook) == 'function' then
        hooks.register(event, hook, hooks.PRIORITY.NORMAL)
      elseif type(hook) == 'table' then
        hooks.register(event, hook[1] or hook.fun, hook.priority)
      end
    end
  end

  if extension.config then configs[extension.name] = extension.config end
end

---Initialize all extensions and merge their variables and configs with user config
---@return string? Error message if initialization failed
M.init = async.wrap(function()
  if not config.extensions or not next(config.extensions) then return end

  for ty, def in pairs(config.extensions) do
    local extension, name, cfg

    if type(ty) == 'number' then
      name = def
    elseif type(ty) == 'string' and type(def) == 'table' then
      name = ty
      cfg = def
    else
      return 'Extension entry must be a string or table'
    end

    if type(name) ~= 'string' then return 'Extension entry must be a string' end

    logger.debug('Loading extension: ' .. name)
    local ok, mod = pcall(require, name)
    if not ok and not name:find '%.' then
      local built_in_name = 'cord.extensions.' .. name
      local ok_built_in, mod_built_in = pcall(require, built_in_name)
      if ok_built_in then
        name = built_in_name
        ok, mod = true, mod_built_in
      end
    elseif not ok and name:find '^cord%.plugins%.' then
      local short_name = name:gsub('^cord%.plugins%.', '')
      local full_name = 'cord.extensions.' .. short_name
      local ok_built_in, mod_built_in = pcall(require, full_name)
      if ok_built_in then
        logger.warn(
          string.format(
            '[\'%s\'] syntax is deprecated, please use \'%s\' instead',
            name,
            short_name
          )
        )
        name = full_name
        ok, mod = true, mod_built_in
      end
    end

    if not ok then return 'Failed to load extension \'' .. name .. '\': ' .. mod end
    extension = mod

    if extension.setup then
      logger.debug('Setting up extension: ' .. name)
      local success, result = pcall(extension.setup, cfg)
      if success and async.is_async(extension.setup) then
        ---@cast result Future
        local val, err = result:await()
        if err then
          success = false
          result = err
        else
          success = true
          result = val
        end
      end
      if not success then return 'Extension \'' .. name .. '\' setup failed: ' .. result end
      if type(result) ~= 'table' then
        return 'Extension \'' .. name .. '\' setup must return a table'
      end
      extension = result
    end

    logger.debug('Registering extension: ' .. name)
    M.register(extension)
  end

  if type(config.variables) ~= 'table' then config.variables = {} end

  for _, extension_vars in pairs(variables) do
    for name, fn in pairs(extension_vars) do
      -- Only add if not already overridden
      if not config.variables[name] then config.variables[name] = fn end
    end
  end

  if type(config.assets) ~= 'table' then config.assets = {} end

  for _, extension_assets in pairs(assets) do
    for name, asset in pairs(extension_assets) do
      -- Only add if not already overridden
      if not config.assets[name] then config.assets[name] = asset end
    end
  end

  local user_config = require('cord').user_config or {}

  local extension_configs = {}
  for _, extension_config in pairs(configs) do
    -- An extension must not attempt to override these inside config
    extension_config.variables = nil
    extension_config.hooks = nil
    extension_config.assets = nil
    extension_config.extensions = nil
    -- deprecated
    extension_config.plugins = nil

    extension_configs = config_util.tbl_deep_extend('keep', extension_configs, extension_config)
  end
  local merged_config = config_util.tbl_deep_extend('force', extension_configs, user_config)
  local final_config = config_util.tbl_deep_extend('force', config.get(), merged_config)

  if not require('cord.api.config').verify(final_config) then return 'Invalid config' end
end)

return M
