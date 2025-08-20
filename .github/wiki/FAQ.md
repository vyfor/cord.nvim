## ❓ FAQ

Here are some common questions about cord.nvim that nobody asked, yet we answered anyway. If you don't find your answer here or in the [Troubleshooting Guide](./Troubleshooting.md), don't hesitate to ask in our [Discord community](https://discord.gg/q9rC4bjCHv) or [GitHub Discussions](https://github.com/vyfor/cord.nvim/discussions)!

> ### Q: What is the minimum required version of Neovim?

Cord is tested with Neovim **0.6.0** or later. Although, we encourage you to use the latest stable version of Neovim, as it provides the best experience and performance. However, if you're running an older version, and find that Cord is not working as expected, please open an issue and we'll try to help you out.

> ### Q: Do I need to install Rust to use Cord?

Nope, you don't need Rust anymore! Cord will automatically download the necessary server binary from GitHub. If you want to build the binary from source, refer to [this](./Build.md) page.

> ### Q: How to see the logs?

There are two ways to see logs:

1. Pass in the desired log level to the `log_level` field in your configuration. Logs at that level and above will be output to `:messages`.

```lua
require 'cord'.setup {
    log_level = '...' -- one of 'trace', 'debug', 'info', 'warn', 'error'
}
```

2. Set the `CORD_LOG_FILE` environment variable to a file path. This will redirect all logs to that file. This is useful for debugging as trace and debug logs can be very verbose and overwhelming in the editor.

> [!NOTE]
> If you were asked to provide logs as part of an issue, you should enable verbose logging via `log_level = 'trace'` and set `CORD_LOG_FILE` env var. Use of relative paths is allowed, e.g. `export CORD_LOG_FILE="./cord.log"`. The log file gets cleared at plugin startup, so keep that in mind.

> ### Q: Can I use a custom name in my Rich Presence?

Yes, you will have to create an application with the desired name in the [Discord Developer Portal](https://discord.com/developers/applications).
Then, copy the application ID and put it in the `editor.client` field in your `cord.nvim` configuration.

Example:
```lua
require 'cord'.setup {
    editor = {
        client = '01234567890123456789'
    }
}
```

> ### Q: Why do I still see Cord's server running in background, even after I've closed Neovim?

Cord's server keeps running intentionally. In fact, this is one of the key design features that sets it apart from similar plugins. It remains active in the background to maintain a continuous connection to Discord, which helps avoid hitting Discord's rate limits on reconnections, especially useful if you often restart Neovim rather than maintaining a single long session. If you prefer not to have it running, you can adjust the `advanced.server.timeout` setting.

> ### Q: I'm using a custom Discord client. Will Cord work with it?

See [Special Environments](./Special-Environments.md#-custom-discord-clients).

> ### Q: Is X plugin or X language supported?

Cord detects different buffers based on their filetype, and occasionally their filename. See the list of supported filetypes [here](https://github.com/vyfor/cord.nvim/blob/master/lua/cord/plugin/activity/mappings.lua). If it's not listed, it usually means one of two things:
- It has not been added yet — feel free to [open an issue](https://github.com/vyfor/cord.nvim/issues/new/choose) and we'll add it.
- It cannot be detected:
  - *Languages* that cannot be detected by filetype or filename alone, have to be configured to be detectable, as explained in [here](https://github.com/vyfor/cord.nvim/wiki/Assets#-tip).
  - *Plugins* are required to override current buffer's `filetype`, or otherwise Cord will not be able to detect it.

> ### Q: Rich Presence updates take a long time to appear in Discord. Why?

Rich Presence updates now take longer to appear because Discord enforces a rate limit on how frequently these updates can be sent. Originally, Discord's documentation allowed one update every 15 seconds—a limit that mostly affected mobile and web clients while desktop users saw instantaneous changes. However, after a recent overhaul of Discord's rich presence interface, this, or a similar rate limit appears to be strictly applied across all platforms, causing the delays you're noticing. From my point of view, this rate limit is ridiculously high, and should be drastically reduced. Perhaps a collaborative effort from the community could make them reconsider their decision, but there's nothing I can do on my end, I'm afraid. See the relevant [discussion](https://github.com/vyfor/cord.nvim/discussions/196).

> ### Q: Why can't I disable timestamps in my Rich Presence? Why are they misbehaving?

It used to work as expected, but I suspect that Discord introduced a bug in recent updates. When you omit timestamps in your activity payload, the absence of a timer is expected at first. However, within seconds the timer reappears and resets to zero on every new activity update. In my testing, this reappearance has been inconsistent—sometimes happening immediately, and other times after a couple of seconds. Interestingly, during the brief period without the timer, updates go through instantly without any apparent rate limiting on the client side. This behavior appears to be due to changes in how Discord handles rich presence updates internally, rather than an issue with the plugin itself. See the relevant [discussion](https://github.com/vyfor/cord.nvim/discussions/196#discussioncomment-12221577).
