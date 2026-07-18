# 🔧 Troubleshooting

## 🛠️ General Steps

1. Update cord.nvim, make sure `:Cord update` is also executed.
2. Double-check your [Discord Activity Privacy settings](https://github.com/vyfor/cord.nvim/assets/92883017/c0c8c410-e90e-425e-bf10-8b59f04f15ce).
3. Enable logging. See [FAQ](./FAQ.md#q-logging-for-debugging).
4. Run `:checkhealth cord` for a config check.
5. Make sure the Discord IPC pipe exists:

   - **Windows:** 
     ```pwsh
     Test-Path \\.\pipe\discord-ipc-0
     ```

   - **Linux/macOS:**
     ```sh
     find /tmp ${XDG_RUNTIME_DIR:+$XDG_RUNTIME_DIR} ${TMPDIR:+$TMPDIR} ${TMP:+$TMP} ${TEMP:+$TEMP} -type s -name 'discord-ipc-*' 2>/dev/null
     ```

   If you get nothing or "False", the pipe does not exist in the expected location.

## 🔁 Plugin Not Loaded (lazy.nvim)

Add `event = 'VeryLazy'` to the plugin spec.

## 🎛️ No Buttons in Rich Presence

Client-sided bug. Join any voice channel and hover over your name.

## ⏱️ Rich Presence Timer Stuck at 00:00

Your system clock might be off. Sync your clock with NTP.

## 🔒 EACCES: Permission Denied

```sh
chmod +x ~/.local/share/nvim/cord/bin/cord
```

## ❓ Still Having Trouble?

If nothing above works:

1. Check the [FAQ](./FAQ.md) for other common questions.
2. Look through [existing GitHub issues](https://github.com/vyfor/cord.nvim/issues).
3. [Open a new issue](https://github.com/vyfor/cord.nvim/issues/new/choose).
   Be clear and include as much detail as possible.

You can also ask in the Discord server if you're unsure.
