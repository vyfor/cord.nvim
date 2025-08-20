local logfile = os.getenv 'CORD_LOG_FILE'

if logfile and logfile ~= '' then
  return require 'cord.api.log.file'
else
  return require 'cord.api.log.notify'
end
