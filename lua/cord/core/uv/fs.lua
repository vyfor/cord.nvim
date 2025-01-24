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
