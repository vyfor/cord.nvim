# 📗 Contribution Guidelines

Thank you for your interest in contributing to cord.nvim! This document provides guidelines for contributing to the project.

---

## TL;DR

- Fork and branch from `master`.
- Style:
  - Lua: format with `stylua` and lint with `luacheck`.
  - Rust: format with `cargo fmt --all` and lint with `cargo clippy --all-targets --all-features`.
- If your changes affect the server component, [build](./Build.md) the binary from source.
- Update docs when your change affects behavior or config.
- Submit a PR with a clear overview and description of your changes. PR title and commits must follow [Conventional Commits](https://www.conventionalcommits.org/) spec.

---

## Getting Started

If you're new to contributing in general, we recommend starting small by improving the documentation or finding missing mappings and adding them to the appropriate [mappings](./lua/cord/plugin/activity/mappings.lua) table.

## Project Overview

### Lua runtime: `lua/cord/`
The Neovim-side implementation: configuration, activity detection, hooks, built-in plugins, and orchestration with the server.

<details>
<summary><strong>Expand</strong></summary>

- `lua/cord/api/`
  - `command.lua`: `:Cord ...` user commands.
  - `config.lua`: Config validation for health checks.
  - `plugin.lua`: Plugin API.
  - `icon.lua`: Icon API.
- `lua/cord/core/`
  - `async/`: Async/Futures helpers on top of coroutines.
  - `uv/`: Thin wrappers over libuv based on the Futures system above.
- `lua/cord/plugin/`
  - `activity/`: Core activity system
    - `mappings.lua`: Buffer/filetype/context mappings.
    - `manager.lua`: Builds/schedules/sends activities; idle handling; queueing.
    - `hooks.lua`: Hook registration and execution used by the manager.
    - `init.lua`: Aggregates the activity subsystem, builds the Activity object.
  - `config/`: Runtime config layer (`init.lua`, `util.lua`) for merge/override logic.
  - `constants/`: Constants and enums used across the plugin.
  - `fs/workspace.lua`: Workspace discovery from root markers (`.git`, `.hg`, `.svn`, etc.).
  - `log/`: Logging backends
    - `init.lua`: Logger facade.
    - `file.lua`: File logger.
    - `notify.lua`: `:messages` logger.
- `lua/cord/plugins/` Built-in plugins
- `lua/cord/server/`
  - `init.lua`: High-level server lifecycle: connect, restart, shutdown, and integration with events.
  - `event/receiver.lua` / `event/sender.lua`: Message bus between Lua and Rust server.
  - `spawn/`: Spawning the server process.
  - `update/`: Fetch/build/update the server executable.
  - `fs/`: Server-related path management.

</details>

### Rust server: `src/`
Discord IPC, message protocol, session management, cross-platform pipes and some other features that require a common server between instances.

<details>
<summary><strong>Expand</strong></summary>

- `cord.rs`: Server runtime: session lifecycle, event loop, and client coordination.
- `cli/`: CLI arguments and error handling.
- `ipc/`
  - `discord/`: Discord RPC client.
  - `pipe/`: Cross-platform pipe layer used to communicate with Neovim Lua plugin.
- `messages/` Events/messages.
  - `events/client/`: Messages from Lua -> server (`connect`, `initialize`, `update_activity`, `clear_activity`, `disconnect`, `shutdown`).
  - `events/server/`: Messages from server -> Lua (`ready`, `log`, `disconnect`).
  - `events/local/`: Internal messaging.
- `presence/` Discord Rich Presence models.
  - `activity.rs`: Activity object.
  - `packet.rs`: Packet with the Activity object.
- `protocol/`
  - `json/` and `msgpack/`: Serialization/deserialization.
- `session/`: Session management (one server for multiple Neovim instances).
- `types/`: Shared types, including the config struct.
- `util/`: Lockfile (to enforce single instance), logger, macros, and other utils.

</details>

### Documentation
- `.github/wiki/`: Documentation source which is published to GitHub Wiki pages.

---

## Local Development

1. Clone the repositoy:
   ```bash
   git clone https://github.com/vyfor/cord.nvim
   ```

2. Load the plugin, this is easiest with a plugin manager (e.g. `lazy.nvim`):

```lua
{
  'vyfor/cord.nvim',
  dir = 'path/to/cloned/cord.nvim',
  -- comment this out if you're working on the server component
  -- see https://github.com/vyfor/cord.nvim/wiki/Build
  build = ':Cord update',
}
```

---

### Logging and debugging

If something work as expected, it's always a good idea to check the logs. See https://github.com/vyfor/cord.nvim/wiki/FAQ#q-how-to-see-the-logs.

## Versioning

- **PATCH**: Non-breaking changes.
- **MINOR**: Breaking changes.
- **MAJOR**: Significant rewrites or fundamental shifts, bumped manually.
