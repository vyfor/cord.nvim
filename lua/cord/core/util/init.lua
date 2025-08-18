local M = {}

function M.url_encode(str)
  return (str:gsub('([^%w%-_%.~])', function(c) return string.format('%%%02X', string.byte(c)) end))
end

return M
