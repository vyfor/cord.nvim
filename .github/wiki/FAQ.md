## ❓ FAQ

Some common questions about cord.nvim that nobody asked, yet we answered anyway. If you don't find your answer here or in the [Troubleshooting Guide](./Troubleshooting.md), don't hesitate to ask in our [Discord community](https://discord.gg/q9rC4bjCHv) or [GitHub Discussions](https://github.com/vyfor/cord.nvim/discussions)!

> ### Q: What is the minimum required version of Neovim?

Cord is compatible with Neovim **0.6.0** or newer.

> ### Q: Do I need to install Rust to use Cord?

No, you don't need Rust anymore.
Cord will download the server binary for you.
If you'd rather build it yourself, check the [Build guide](./Build.md).

> ### Q: How to see the logs?

You can check the logs in two ways:

1. Set a log level in your config. Messages at that level (and higher) will show up in `:messages`.

```lua
require 'cord'.setup {
    log_level = '...' -- one of 'trace', 'debug', 'info', 'warn', 'error'
}
```

2. Set the `CORD_LOG_FILE` environment variable to a file path. This saves logs to a file instead of spamming your editor, which is especially useful when using `trace` or `debug` levels.

> [!NOTE]
> If you were asked to provide logs as part of an issue, you should enable verbose logging via `log_level = 'trace'` and set `CORD_LOG_FILE` environment variable. Use of relative paths is allowed, e.g. `export CORD_LOG_FILE="./cord.log"`. The log file gets cleared at plugin startup, so keep that in mind.

> ### Q: Can I use a custom name in my Rich Presence?

To do this, you will have to create an application with the desired name in the [Discord Developer Portal](https://discord.com/developers/applications).
Then, copy its application ID and put it in the `editor.client` field:

```lua
require 'cord'.setup {
    editor = {
        client = '01234567890123456789'
    }
}
```

> ### Q: Why do I still see Cord's server running in background, even after I've closed Neovim?

That's intentional.
Cord's server keeps running to maintain a single continuous connection to Discord and avoid hitting rate limits from reconnecting too often.
This is especially useful if you restart Neovim a lot.
If you'd rather shut it down sooner, there's the `advanced.server.timeout` option.

> ### Q: I'm using a custom Discord client. Will Cord work with it?

See [Special Environments](./Special-Environments.md#-custom-discord-clients).

> ### Q: Is X plugin or X language supported?

Cord detects buffers mostly by filetype (and sometimes by filename).
Check the list of supported filetypes [here](https://github.com/vyfor/cord.nvim/blob/master/lua/cord/plugin/activity/mappings.lua).


If the language or plugin you use isn't found on the list, please [open an issue](https://github.com/vyfor/cord.nvim/issues/new/choose).

Just keep in mind:
- **Languages** that don't expose a clear filetype or filename need extra setup (see [this page](https://github.com/vyfor/cord.nvim/wiki/Assets#-tip)).
- **Plugins** need to override the buffer's `filetype`, otherwise Cord won't be able to recognize them.

> ### Q: Rich Presence updates take a long time to appear in Discord. Why?

After the recent update, Discord started to strictly rate limit how often your Rich Presence can update. The exact numbers aren't known yet.

**UPD:** Through some testing I found that the rate limit moves between an 8/12s alternating cooldown and a 2-per-20s burst window depending on how long the presence has been running.

If your status updates stop appearing, it might be because you've been rate limited. To make matters worse, Discord does not seem to "queue" updates anymore. To mitigate this, Cord provides an `advanced.discord.sync` feature. It's enabled by default so we strongly encourage you to keep up to date:

```lua
require 'cord'.setup {
    advanced = {
        discord = {
            sync = {
                enabled = true,
                mode = 'periodic', -- Periodically resends last activity to resume "stuck" activities
                interval = 12000,
            },
        }
    },
}
```

Alternatively, you can use `mode = 'defer'` to throttle your updates more strictly and try to avoid being rate limited in the first place.

> [!NOTE]
> I also have a hypothesis that Discord's internal state is somehow getting desynced. It *may* be thinking an update isn't necessary even when the displayed activity is out of date. To prevent this, the server now "pads" fields with whitespaces (can be disabled via `advanced.discord.sync.pad`) to force Discord to recognize the update as new data. Even in case my hypothesis proves wrong, it carries no harm, and seems to provide better results in my observations. It all sounds a little silly but I can't think of any reason *why* would they intentionally remove queuing in the first place, thus, this must be some kind of a bug.


This issue is not Cord's fault.
See the [discussion](https://github.com/vyfor/cord.nvim/discussions/321) for details.

> ### Q: Why can't I disable timestamps in my Rich Presence? Why are they misbehaving?

Disabling timestamps was previously possible but Discord appears to have introduced a bug.
If you omit the timestamp now, it starts to misbehave, with the timer initially briefly disappearing but then reappearing and resetting to zero whenever the activity updates. No can do on our end.


Not caused by Cord.
See the [discussion](https://github.com/vyfor/cord.nvim/discussions/196#discussioncomment-12221577) for details.
