# 🔧 Troubleshooting

### Rich Presence is not shown in Discord
1. Ensure that `cord.nvim` is loaded (try running a user command, e.g. `:Cord status`)
2. Confirm that your Activity Privacy settings are [enabled](https://github.com/vyfor/cord.nvim/assets/92883017/c0c8c410-e90e-425e-bf10-8b59f04f15ce)
3. Set [`advanced.plugin.log_level`](https://github.com/vyfor/cord.nvim/wiki/Configuration#️-advanced) to `vim.log.levels.TRACE` and check `:messages` for logs
4. Verify that the Discord IPC pipe exists:

   **Windows:**
   ```pwsh
   Test-Path \\.\pipe\discord-ipc-0
   ```

   **Unix:**
   ```sh
   find /tmp /var/run /run -type s -name 'discord-ipc-*' 2>/dev/null
   ```

### No buttons are shown in the Rich Presence
- After the recent UI update related to user profiles, there's a client-sided bug on Discord where you yourself can't see the buttons on your own rich presence. You can either join a voice channel and hover over it to see the buttons, or ask somebody else to confirm if they can see them

### Rich Presence timer is stuck at 00:00
- This issue is usually resolved by syncing your system date and timezone

### Running inside WSL
- WSL doesn't expose Windows pipes by default. In order to do so, install [socat](https://www.kali.org/tools/socat) and [npiperelay](https://github.com/jstarks/npiperelay/), then alias nvim to expose the pipe as done in this [guide](https://gist.github.com/mousebyte/af45cbecaf0028ea78d0c882c477644a#aliasing-nvim):
    > ```sh
    > nvim () {
    >     pidof socat > /dev/null 2>&1
    >     if ! $? -eq 0; then
    >         socat UNIX-LISTEN:/tmp/discord-ipc-0,fork \
    >           EXEC:"npiperelay.exe //./pipe/discord-ipc-0"&
    >     fi
    >     command nvim "$@"
    > }
    > ```

### Running on a remote server
- Expose the Discord IPC socket over SSH using this [guide](https://carlosbecker.com/posts/discord-rpc-ssh/)

### Is your issue not listed?
- Please open a new [issue](https://github.com/vyfor/cord.nvim/issues/new/choose)