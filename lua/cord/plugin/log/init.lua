local logfile = os.getenv 'CORD_LOG_FILE'

if logfile and logfile ~= '' then
  return require 'cord.plugin.log.file'
else
  return require 'cord.plugin.log.notify'
end
