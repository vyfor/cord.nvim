local M = {}

local language = require 'cord.mappings.language'
local file_browser = require 'cord.mappings.file_browser'
local lsp_manager = require 'cord.mappings.lsp_manager'
local plugin_manager = require 'cord.mappings.plugin_manager'
local docs = require 'cord.mappings.docs'
local vcs = require 'cord.mappings.vcs'
local dashboard = require 'cord.mappings.dashboard'

M.get = function(filetype, filename)
  local result = language.get(filetype, filename)
  if result then return 'language', result[1], result[2] end

  result = file_browser.get(filetype)
  if result then return 'file_browser', result[1], result[2] end

  result = plugin_manager.get(filetype)
  if result then return 'plugin_manager', result[1], result[2] end

  result = lsp_manager.get(filetype)
  if result then return 'lsp', result[1], result[2] end

  result = docs.get(filetype)
  if result then return 'docs', result[1], result[2] end

  result = vcs.get(filetype)
  if result then return 'vcs', result[1], result[2] end

  result = dashboard.get(filetype)
  if result then return 'dashboard', result[1], result[2] end

  return 'language', 'text', filetype
end

return M
