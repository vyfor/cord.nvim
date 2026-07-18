## ❓ FAQ

Some common questions about cord.nvim that nobody asked, yet we answered anyway.

<details>
<summary><h4>What is the minimum required version of Neovim?</h4></summary>

Cord is compatible with Neovim **0.6.0** or newer.

</details>

<details>
<summary><h4>Do I need to install Rust to use Cord?</h4></summary>

No, you don't need Rust anymore.
Cord will download the server binary for you.
If you'd rather build it yourself, check the [Build guide](./Build.md).

</details>

<details>
<summary><h4>How to see the logs?</h4></summary>

Cord's notifications/messages/logs are controlled by the `CORD_LOG_LEVEL` env var OR `log_level` configuration option. Usually, you want this value to be set to one of `off`, `error`, `warn`, `info`.

```lua
require('cord').setup {
    log_level = '...' -- one of 'trace', 'debug', 'info', 'warn', 'error'
}
```

Where logs go depends on the presence of the `CORD_LOG_FILE` environment variable. The log file is cleared at plugin startup.

</details>

<details>
<summary><h4>Logging for debugging</h4></summary>

For comfortable debugging, have the log file open and periodically reload it (`:e`).

```sh
CORD_LOG_LEVEL="trace" CORD_LOG_FILE="cord.log" nvim cord.log
```

</details>

<details>
<summary><h4>Can I use a custom name in my Rich Presence?</h4></summary>

To do this, you will have to create an application with the desired name in the [Discord Developer Portal](https://discord.com/developers/applications).
Then, copy its application ID and put it in the `editor.client` field:

```lua
require 'cord'.setup {
    editor = {
        client = '01234567890123456789'
    }
}
```

</details>

<details>
<summary><h4>Why do I still see Cord's server running in background, even after I've closed Neovim?</h4></summary>

That's intentional.
Cord's server keeps running to maintain a single continuous connection to Discord and avoid hitting rate limits from reconnecting too often.
This is especially useful if you restart Neovim a lot.
If you'd rather shut it down sooner, there's the `advanced.server.timeout` option.

</details>

<details>
<summary><h4>I'm using a custom Discord client. Will Cord work with it?</h4></summary>

See [Special Environments](./Special-Environments.md#-custom-discord-clients).

</details>

<details>
<summary><h4>Is X plugin or X language supported?</h4></summary>

Cord detects buffers mostly by filetype (and sometimes by filename).
Check the list of supported filetypes [here](https://github.com/vyfor/cord.nvim/blob/master/lua/cord/plugin/activity/mappings.lua).

If the language or plugin you use isn't found on the list, please [open an issue](https://github.com/vyfor/cord.nvim/issues/new/choose).

Just keep in mind:
- **Languages** that don't expose a clear filetype or filename need extra setup (see [this page](https://github.com/vyfor/cord.nvim/wiki/Assets#-tip)).
- **Plugins** need to override the buffer's `filetype`, otherwise Cord won't be able to recognize them.

</details>

<details>
<summary><h4>Rich Presence updates take a long time to appear in Discord. Why?</h4></summary>

After the recent update, Discord started to strictly rate limit how often your Rich Presence can update. The exact numbers aren't known yet.

**UPD:** Through some testing I found that the rate limit moves between an 8/12s alternating cooldown and a 2-per-20s burst window depending on how long the presence has been running.

Crucially, it looks like Discord no longer "queues" updates; if you're rate-limited, any updates seem to be simply dropped until the cooldown expires. To fix this, Cord includes an `advanced.discord.sync` feature (enabled by default). It ensures your status stays current by periodically resending your activity on every `interval`:

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

Alternatively, you can use `mode = 'defer'`. Instead of sending updates immediately, this mode holds onto them and waits for the next `interval` before sending them out. It's a stricter way to throttle updates to avoid triggering a rate limit in the first place.

> [!NOTE]
> I also suspect that Discord's internal state is somehow getting desynced. It seems like the server might be "deciding" an update isn't necessary because it doesn't recognize the data as new, even when the displayed activity is stuck or out of date.
> To fix this, the plugin now "pads" fields with whitespaces (can be disabled via `advanced.discord.sync.pad`) to hopefully trick Discord into seeing the update as brand new data. Even if my hypothesis is inaccurate, this "hack" carries no harm and, in my testing, consistently leads to more reliable updates. It's a bit of a workaround, but since Discord removed update queuing, this seems to be the best way to handle what looks like a bug on their end.

This issue is not Cord's fault.

See the [discussion](https://github.com/vyfor/cord.nvim/discussions/321) for details.

</details>

<details>
<summary><h4>Why can't I disable timestamps in my Rich Presence? Why are they misbehaving?</h4></summary>

Disabling timestamps was previously possible but Discord appears to have introduced a bug.
If you omit the timestamp now, it starts to misbehave, with the timer initially briefly disappearing but then reappearing and resetting to zero whenever the activity updates. No can do on our end.

Not caused by Cord.
See the [discussion](https://github.com/vyfor/cord.nvim/discussions/196#discussioncomment-12221577) for details.

</details>
