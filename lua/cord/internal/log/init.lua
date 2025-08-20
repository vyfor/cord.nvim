local logfile = os.getenv 'CORD_LOG_FILE'

if logfile and logfile ~= '' then
  return require 'cord.internal.log.file'
else
  return require 'cord.internal.log.notify'
end
