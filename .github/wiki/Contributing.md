# 📗 Contribution Guidelines

Thank you for your interest in contributing to cord.nvim! This document provides guidelines for contributing to the project.

## Project Structure

cord.nvim consists of several key components:

- **Lua Plugin**: The main Neovim plugin code located in `lua/cord/`.
- **Server Component**: The server responsible for managing multiple Neovim instances and IPC communication located in `src/`.
- **Documentation**: Wiki pages are pulled from `.github/wiki/`.
- **Assets**: Icons are now stored in a separate repository, [vyfor/icons](https://github.com/vyfor/icons).

## Versioning

We use a modified versioning scheme (MAJOR.MINOR.PATCH):

- **PATCH**: Backward compatible changes (features and fixes)
- **MINOR**: Breaking changes
- **MAJOR**: Significant architectural changes

## How to Contribute

### Contributing to the Lua Plugin

Cord detects different buffers based on current buffer's filetype, and on rare occasions, its filename. If it's not listed, it's either:
- Not yet added to Cord. You can submit a PR, or open an issue and let us handle it.
    - The code responsible for this is in `lua/cord/plugin/activity/mappings.lua`.
- Cannot be detected by Cord.
    - This is often the case for plugins, so make sure the plugin you're contributing overrides current buffer's filetype option so that it can be detected.

If you'd like to contribute a built-in plugin that might be useful for many users, you can add it in `lua/cord/plugins/`.

### Pull Requests

1. Fork and create a feature branch from `master`
2. Follow our code style:
   - Rust: Use `rustfmt` and `clippy`
   - Lua: Use `stylua`
3. Test your changes
4. Submit a PR with a title following [Conventional Commits](https://www.conventionalcommits.org/)

Examples:
```
feat(icons): add support for custom icon themes

fix(server): handle Discord disconnection gracefully

feat(server)!: change communcation protocol\n\nBREAKING CHANGE: This change requires users to update their configuration.
```

## Getting Help

- Join our [Discord server](https://discord.gg/q9rC4bjCHv)
- Check the [FAQ](./FAQ.md)
- Browse [Discussions](https://github.com/vyfor/cord.nvim/discussions)

Thank you for contributing to cord.nvim!
