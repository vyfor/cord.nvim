local hooks = require 'cord.plugin.activity.hooks'
local config = require 'cord.plugin.config'
local logger = require 'cord.plugin.log'

local M = {}

local plugins = {}
local variables = {}
local assets = {}
local configs = {}

---@class CordPlugin
---@field name string Plugin name
---@field description? string Plugin description
---@field variables? table<string,function> Variables to add
---@field hooks? table<string,function>|{fun: function, priority: number} Hooks to register
---@field assets? table<string,string|CordAssetConfig> Assets to add
---@field config? table Configuration to merge

---Register a plugin
---@param plugin CordPlugin Plugin definition table
function M.register(plugin)
  if not plugin.name then
    error 'Plugin must have a name'
    return
  end

  if plugins[plugin.name] then return end

  plugins[plugin.name] = plugin

  if plugin.variables then variables[plugin.name] = plugin.variables end

  if plugin.assets then assets[plugin.name] = plugin.assets end

  if plugin.hooks then
    for event, hook in pairs(plugin.hooks) do
      if type(hook) == 'function' then
        hooks.register(event, hook, hooks.PRIORITY.NORMAL)
      elseif type(hook) == 'table' then
        hooks.register(event, hook[1] or hook.fun, hook.priority)
      end
    end
  end

  if plugin.config then configs[plugin.name] = plugin.config end
end

---Initialize all plugins and merge their variables and configs with user config
---@return string? Error message if initialization failed
function M.init()
  if not config.plugins or #config.plugins == 0 then return end

  for _, plugin in ipairs(config.plugins) do
    local plugin_name
    local plugin_config

    if type(plugin) == 'string' then
      plugin_name = plugin
      logger.debug('Loading plugin: ' .. plugin_name)
      local ok, mod = pcall(require, plugin_name)
      if not ok then return 'Failed to load plugin \'' .. plugin_name .. '\': ' .. mod end
      plugin = mod
    elseif type(plugin) == 'table' then
      plugin_name = plugin[1] or plugin.name
      plugin_config = plugin.config
      logger.debug('Loading plugin with config: ' .. plugin_name)
      local ok, mod = pcall(require, plugin_name)
      if not ok then return 'Failed to load plugin \'' .. plugin_name .. '\': ' .. mod end
      plugin = mod
    else
      return 'Plugin must be a string or table'
    end

    if plugin.setup then
      logger.debug('Setting up plugin: ' .. plugin_name)
      local success, result = pcall(plugin.setup, plugin_config)
      if not success then return 'Plugin \'' .. plugin_name .. '\' setup failed: ' .. result end
      if type(result) ~= 'table' then
        return 'Plugin \'' .. plugin_name .. '\' setup must return a table'
      end
      plugin = result
    end

    logger.debug('Registering plugin: ' .. plugin_name)
    M.register(plugin)
  end

  if type(config.variables) ~= 'table' then config.variables = {} end

  for _, plugin_vars in pairs(variables) do
    for name, fn in pairs(plugin_vars) do
      -- Only add if not already overridden
      if not config.variables[name] then config.variables[name] = fn end
    end
  end

  if type(config.assets) ~= 'table' then config.assets = {} end

  for _, plugin_assets in pairs(assets) do
    for name, asset in pairs(plugin_assets) do
      -- Only add if not already overridden
      if not config.assets[name] then config.assets[name] = asset end
    end
  end

  local user_config = require('cord').user_config or {}

  local plugin_configs = {}
  for _, plugin_config in pairs(configs) do
    -- A plugin must not attempt to override these inside config
    plugin_config.variables = nil
    plugin_config.hooks = nil
    plugin_config.assets = nil
    plugin_config.plugins = nil

    plugin_configs = vim.tbl_deep_extend('keep', plugin_configs, plugin_config)
  end
  local merged_config = vim.tbl_deep_extend('force', plugin_configs, user_config)
  local final_config = vim.tbl_deep_extend('force', config.get(), merged_config)

  if not require('cord.plugin.config.util'):validate(final_config) then return 'Invalid config' end
end

return M
