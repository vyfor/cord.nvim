# ⏳ Async Configuration

Cord provides an asynchronous runtime that allows for non-blocking I/O operations within your configuration. This enables you to perform expensive tasks - running commands, reading files, or fetching APIs in the background.

This guide explains how to leverage the `cord.core.async` module to write non-blocking configuration functions.

## 📖 Table of Contents
  - [Core Concepts](#-core-concepts)
  - [Basic Usage](#-basic-usage)
  - [Caching](#-caching)
  - [Error Handling](#-error-handling)
  - [Concurrent Execution](#-concurrent-execution)
  - [Available Modules](#-available-modules)
  - [Examples](#-examples)
  - [Common Pitfalls](#-common-pitfalls)

---

## 🧠 Core Concepts

The async system is built on top of Lua coroutines and exposes a Promise-like API.

-  **Async Function**: A function wrapped with `async.wrap`. When called, it executes within a protected coroutine.
-  **Future**: An object representing the eventual result of an asynchronous operation.

For advanced use cases, `async.run` can be used to spawn a coroutine an enter async context from within a regular function.

## 🚀 Basic Usage

To use async features in your config, you must wrap your function using `async.wrap`.

```lua
variables = {
  git_branch = async.wrap(function(opts)
    -- runs in a coroutine
    return 'main' 
  end)
}
```

### Awaiting results

When you call an async operation (like `readfile` or `spawn`), it returns a `Future`. Calling `:await()` on this future suspends execution until the data is ready.

```lua
local content = fs.readfile('config.json'):await() 
```

> [!IMPORTANT]
> You cannot use `:await()` inside a regular function. It must be called inside a coroutine context, which is ensured by `async.wrap`.

---

## 💾 Caching

Since Cord re-runs your config frequently to detect changes (on buffer change, cursor move, etc.), repeating expensive async operations is wasteful.

Cord provides a global cache instance available as `opts.cache` in your functions.

```lua
-- fetch value, OR compute and cache it for 60 seconds
local value = opts.cache:get_or_compute(key, 60, function()
  return do_expensive_task():await()
end)
```

> [!TIP]
> The cache is global. Always prefix your keys *(e.g., with `opts.workspace_dir` if data is scoped to a workspace)* to prevent collisions. Alternatively, use a local cache instance instead.

---

## 🛡️ Error Handling

Asynchronous operations can fail. The `await()` method handles this by returning a pair of values: `(result, error)`, similar to standard Lua I/O functions.

### Handling Errors Gracefully

```lua
local content, err = fs.readfile('non_existent.txt'):await()

if err then
  -- handle the error or return a fallback
  return 'Error: ' .. err
end

return content
```

### Using `unwrap()`

If you are certain an operation will succeed, or if you want the function to crash/propagate the error immediately, you can use `:unwrap()` instead.

```lua
-- will throw an error if the read fails
local content = fs.readfile('critical_data.txt'):unwrap()
```

---

## ⚡ Concurrent Execution

When you call an async function *without* awaiting it, it starts executing immediately in the background. To run multiple operations concurrently, start them all first, then use `Future.all` to wait for the results.

> [!NOTE]
> Unlike "lazy" async models (like Rust or Python) where a task only starts when you await it, Cord's model is **eager**. The task begins the moment the function is called. This is why simply storing futures in variables is enough to start them running in parallel.

**Sequential (Slow):**
```lua
-- execution is paused at line 1 until branch is ready
local branch = get_branch():await()
-- only then does get_status start running
local status = get_status():await()
-- time = time(branch) + time(status)
```

**Concurrent (Fast):**
```lua
local Future = require('cord.core.async.future')

-- both operations start running immediately in the background
local branch_fut = get_branch()
local status_fut = get_status()

-- now we wait for both to finish
local results = Future.all({ branch_fut, status_fut }):await()
local branch, status = results[1], results[2]
-- time = max(time(branch), time(status))
```

### Fire and Forget

Because execution is eager, you can start an operation and simply return without awaiting it. This is useful for side effects like logging or updating an external file—where you don't care about the result and don't want to wait for it.

```lua
-- the write starts in the background and we return 'Done' immediately
fs.writefile('log.txt', 'Activity updated'):catch(function(err)
  print('Logging failed: ' .. err)
end)

return 'Done'
```

---

## 📦 Available Modules

Cord exposes internal libuv wrappers for common tasks via `cord.core.uv`.

### `cord.core.uv.process` (Subprocess)
Used for running external commands (git, gh, curl, etc.).

- `spawn(opts)`: Spawns a process. Returns a Future resolving to `{ code, signal, stdout, stderr, pid }`.
  - `opts.cmd` (string): Command to run
  - `opts.args` (table): Command arguments
  - `opts.cwd` (string): Working directory
  - `opts.env` (table): Environment variables

### `cord.core.uv.fs` (Filesystem)
Async wrappers for file operations. All return Futures.

**High-Level:**
- `readfile(path)`: Reads entire file. Returns content string.
- `writefile(path, data)`: Writes data to file. Returns bytes written.
- `mkdirp(path, [mode])`: Recursively creates directories.
- `unlink(path)`: Deletes a file.
- `rename(old, new)`: Renames/moves a file.
- `stat(path)`: Returns file stats `{ size, mtime, type, ... }`.

**Low-Level:**
- `openfile(path, flags)`: Opens file, returns file descriptor (fd).
- `read(fd, size)`: Reads from fd.
- `write(fd, data)`: Writes to fd.
- `closefile(fd)`: Closes fd.

### `cord.core.uv.pipe` (IPC)
For communicating with sockets/pipes.

- `IPC.new()`: Creates a new IPC instance.
- `ipc:connect(path)`: Connects to a named pipe/socket.
- `ipc:read()`: Reads a chunk of data.
- `ipc:write(data)`: Writes data.
- `ipc:close()`: Closes connection.

---

## 💡 Examples

### Async Git Branch
 
Fetch the current Git branch asynchronously with caching.
 
```lua
local async = require('cord.core.async')
local process = require('cord.core.uv.process')
 
require('cord').setup {
  variables = {
    git_branch = async.wrap(function(opts)
      -- run git command only once every 30 seconds
      return opts.cache:get_or_compute(opts.workspace_dir .. ':branch', 30, function()
        local result, err = process.spawn({
          cmd = 'git',
          args = { 'branch', '--show-current' },
          cwd = opts.workspace_dir,
        }):await()
 
        if err or result.code ~= 0 then
          -- we must return a non-nil value
          return false
        end
        return vim.trim(result.stdout)
      end)
    end),
  },
  text = {
    -- use the variable if available, otherwise fallback
    workspace = async.wrap(function(opts)
      local branch = opts:git_branch():await()
      if branch then return string.format('In %s (%s)', opts.workspace, branch) end
      return string.format('In %s', opts.workspace)
    end)
  },
}
```

### Dynamic Button URL
 
Display a button with a URL that is only available if the repository is not private. Since repository privacy rarely changes, we cache this for 5 minutes.
 
```lua
local async = require('cord.core.async')
local process = require('cord.core.uv.process')
 
require('cord').setup {
  buttons = {
    {
      label = 'View Repository',
      url = async.wrap(function(opts)
        -- check visibility only once every 5 minutes
        local is_private = opts.cache:get_or_compute(opts.workspace_dir .. ':is_repo_private', 300, function()
          local result = process.spawn({
            cmd = 'gh',
            args = { 'repo', 'view', '--json', 'isPrivate', '--template', '{{.isPrivate}}' },
            cwd = opts.workspace_dir
          }):await()
  
          -- assume private if command fails
          if not result or result.code ~= 0 then
            -- we must return a non-nil value
            return false
          end
          return vim.trim(result.stdout)
        end)
  
        if is_private == 'true' then return end -- hide button for private repos
        
        return opts.repo_url
      end)
    }
  }
}
```

### ⚠️ Common Pitfalls

1.  **Blocking Calls inside Async Wrapper**:
    Wrapping a function with `async.wrap` does not magically make existing blocking APIs (like `vim.fn.system`, `io.popen`, or `os.execute`) non-blocking. These functions will still freeze the UI. You must use the provided `cord.core.uv` modules or write your own async wrappers. In contrast, if your task does not involve I/O, there is no need to use async at all.

2.  **Forgetting `async.wrap`**:
    Calling `:await()` requires a running coroutine. If you try to use it in a standard Lua function, you will receive an error.

3.  **Spawning Processes Too Frequently**:
    Cord may run your configuration functions very frequently (depending on your config). Spawning a process dozens of times a minute is expensive. Please [cache](#-caching) your results to avoid this.
