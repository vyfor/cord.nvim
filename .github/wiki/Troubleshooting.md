# 🔧 Troubleshooting

### Failed to update to v2
Lazy may be unable to update to the new branch from the `master` branch, in which case you can force a fresh install of the plugin with the following command:
  
- **Unix:**
    ```sh
    rm -rf ~/.local/share/nvim/lazy/cord.nvim
    ```

- **Windows:**
    ```pwsh
    rm \"$env:LOCALAPPDATA/nvim-data/lazy/cord.nvim\" -r -force
    ```

### Rich Presence is not shown in Discord
1. Confirm that you're not using the web version of Discord or a custom client, or have taken measures to expose the pipe.
2. Ensure that `cord.nvim` is loaded (try running a user command, e.g. `:Cord status`)
4. Check your [Activity Privacy](https://github.com/vyfor/cord.nvim/assets/92883017/c0c8c410-e90e-425e-bf10-8b59f04f15ce) settings
5. Set [`advanced.plugin.log_level`](./Configuration.md#️-advanced) to `vim.log.levels.TRACE` and check `:messages` for logs
6. Verify that the Discord IPC pipe exists:

   **Windows:**
   ```pwsh
   Test-Path \\.\pipe\discord-ipc-0
   ```

   **Unix:**
   ```sh
   find /tmp /var/run /run -type s -name 'discord-ipc-*' 2>/dev/null
   ```
7. Run `:checkhealth cord`

### No buttons are shown in the Rich Presence
- After the recent UI update related to user profiles, there's a client-sided bug on Discord where you yourself can't see the buttons on your own rich presence. You can either join a voice channel and hover over it to see the buttons, or ask somebody else to confirm if they can see them

### Rich Presence timer is stuck at 00:00
- This issue is usually resolved by syncing your system date and timezone

### Running inside WSL
- WSL doesn't expose Windows pipes by default. In order to do so, install [socat](https://www.kali.org/tools/socat) and [npiperelay](https://github.com/jstarks/npiperelay/), then alias nvim to expose the pipe as done in this [guide](https://gist.github.com/mousebyte/af45cbecaf0028ea78d0c882c477644a#aliasing-nvim):
    > ```sh
    > nvim() {
    >     if ! pidof socat > /dev/null 2>&1; then
    >         [ -e /tmp/discord-ipc-0 ] && rm -f /tmp/discord-ipc-0
    >         socat UNIX-LISTEN:/tmp/discord-ipc-0,fork \
    >             EXEC:\"/mnt/c/path/to/npiperelay.exe //./pipe/discord-ipc-0\" &
    >         fi
    > 
    >         if [ $# -eq 0 ]; then
    >             command nvim
    >         else
    >             command nvim \"$@\"
    >         fi
    >     fi
    > }
    ```

### Running on a remote server
- Expose the Discord IPC socket over SSH using this [guide](https://carlosbecker.com/posts/discord-rpc-ssh/)

### Is your issue not listed?
- Please open a new [issue](https://github.com/vyfor/cord.nvim/issues/new/choose)