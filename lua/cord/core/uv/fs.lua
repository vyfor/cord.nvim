local Future = require 'cord.core.async.future'
local uv = vim.loop or vim.uv

local M = {}

function M.stat(path)
  return Future.new(function(resolve, reject)
    uv.fs_stat(path, function(err, stat)
      if err then
        reject(err)
        return
      end
      resolve(stat)
    end)
  end)
end

function M.unlink(path)
  return Future.new(function(resolve, reject)
    uv.fs_unlink(path, function(err)
      if err then
        reject(err)
        return
      end
      resolve(true)
    end)
  end)
end

function M.mkdir(path, mode)
  return Future.new(function(resolve, reject)
    uv.fs_mkdir(path, mode or 511, function(err) -- 511 = 0777 in octal
      if err then
        reject(err)
        return
      end
      resolve(true)
    end)
  end)
end

function M.mkdirp(path, mode)
  return Future.new(function(resolve, reject)
    local function create_dirs(dirs, index)
      if index > #dirs then
        resolve(true)
        return
      end

      local current_path = table.concat(dirs, '/', 1, index)

      uv.fs_stat(current_path, function(stat_err, stat)
        if not stat_err and stat and stat.type == 'directory' then
          create_dirs(dirs, index + 1)
          return
        end

        uv.fs_mkdir(current_path, mode or 511, function(mk_err)
          if mk_err and not mk_err:match('EEXIST') then
            reject(mk_err)
          else
            create_dirs(dirs, index + 1)
          end
        end)
      end)
    end

    local parts = {}
    for part in path:gmatch('[^/]+') do
      table.insert(parts, part)
    end

    if path:sub(1, 1) == '/' then
      table.insert(parts, 1, '')
    end

    if #parts == 0 then
      resolve(true)
      return
    end

    create_dirs(parts, 1)
  end)
end

function M.rename(old_path, new_path)
  return Future.new(function(resolve, reject)
    uv.fs_rename(old_path, new_path, function(err)
      if err then
        reject(err)
        return
      end
      resolve(true)
    end)
  end)
end

function M.readfile(path)
  return Future.new(function(resolve, reject)
    uv.fs_open(path, 'r', 438, function(err, fd) -- 438 = 0666 in octal
      if err then
        reject(err)
        return
      end

      uv.fs_fstat(fd, function(err, stat)
        if err then
          uv.fs_close(fd)
          reject(err)
          return
        end

        uv.fs_read(fd, stat.size, 0, function(err, data)
          uv.fs_close(fd)
          if err then
            reject(err)
            return
          end
          resolve(data)
        end)
      end)
    end)
  end)
end

function M.writefile(path, data)
  return Future.new(function(resolve, reject)
    uv.fs_open(path, 'w', 438, function(err, fd) -- 438 = 0666 in octal
      if err then
        reject(err)
        return
      end

      uv.fs_write(fd, data, 0, function(err, bytes_written)
        uv.fs_close(fd)
        if err then
          reject(err)
          return
        end
        resolve(bytes_written)
      end)
    end)
  end)
end

function M.chmod(path, mode)
  return Future.new(function(resolve, reject)
    uv.fs_chmod(path, mode, function(err)
      if err then
        reject(err)
        return
      end
      resolve(true)
    end)
  end)
end

return M
