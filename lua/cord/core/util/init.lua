local M = {}

function M.url_encode(str)
  return (str:gsub('([^%w%-_%.~])', function(c) return string.format('%%%02X', string.byte(c)) end))
end

function M.tbl_flatten(t)
  local result = {}

  local function flatten(sub)
    for _, v in ipairs(sub) do
      if type(v) == 'table' then
        flatten(v)
      elseif v then
        table.insert(result, v)
      end
    end
  end

  flatten(t)
  return result
end

return M
