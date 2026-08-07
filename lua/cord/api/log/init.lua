local logfile = os.getenv 'CORD_LOG_FILE'

local module
if logfile and logfile ~= '' then
  module = require 'cord.api.log.file'
else
  module = require 'cord.api.log.notify'
end

local env_level = os.getenv 'CORD_LOG_LEVEL'
local env_log_level = nil
if env_level and env_level ~= '' then
  local level = tonumber(env_level)
  if not level then
    level = vim.log.levels[string.upper(env_level)]
  end
  if level then
    env_log_level = level
    module.set_level(level)
    module.set_level = function() end
  end
end
module.env_log_level = env_log_level

return module
