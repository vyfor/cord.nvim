# 🔧 Troubleshooting

Having trouble with Cord? This guide provides solutions to common problems and scenarios you might encounter.

## 🛠️ General Troubleshooting Steps

If you're experiencing issues, try these general steps first:

1.  Restart Neovim, ensure that Cord is loaded, and Discord is running. Make sure both Cord and its server (`:Cord update`) are up-to-date.
2.  Verify your Discord [Activity Privacy settings](https://github.com/vyfor/cord.nvim/assets/92883017/c0c8c410-e90e-425e-bf10-8b59f04f15ce) are enabled to allow Rich Presence to be displayed.
3.  Set `log_level = vim.log.levels.TRACE` in your `cord.setup()` configuration. Then check `:messages` for detailed logs that might indicate the problem. Remember to revert to a less verbose log level (e.g., `vim.log.levels.WARN` or `vim.log.levels.OFF`) after troubleshooting.
4. Run `:checkhealth cord`. This command performs health checks and can identify common configuration or environment issues.
5.  Check if the Discord IPC pipe exists on your system:
    - **Windows (PowerShell):** `Test-Path \\.\pipe\discord-ipc-0`
    - **Unix (Bash):** `find /tmp /var/run /run -type s -name 'discord-ipc-*' 2>/dev/null`
    If the command returns nothing or "False", the pipe may not be available.

If these general steps don't resolve your issue, look for specific problems and solutions below.

## 🎛️ No Buttons Showing in Rich Presence

There's a known client-side bug in Discord related to user profiles. *You may not be able to see buttons on your own Rich Presence*.

Ask a friend or someone else to check your Discord profile to see if they can see the buttons on your Rich Presence. Buttons *are* visible to others even if you can't see them yourself.

Another workaround to check buttons yourself is to join a voice channel in Discord and hover over your name in the voice channel list. Buttons appear in the hover tooltip.

## ⏱️ Rich Presence Timer Stuck at 00:00

This issue is almost always caused by an incorrect system date, time, or timezone setting on your computer.

Ensure your system date, time, and timezone are set correctly and synchronized** with a reliable time source (e.g., using NTP - Network Time Protocol).  Operating systems usually have settings to automatically sync time.

## 💻 Running Cord in Specific Environments

### 🌐 Using Discord in a Browser

Use [arrpc](https://github.com/OpenAsar/arrpc). arrpc creates a communication bridge that allows Cord to send presence updates to web-based Discord. Follow arrpc's setup instructions carefully.

### 🐧 Running Inside WSL (Windows Subsystem for Linux)

> WSL (Windows Subsystem for Linux) does not directly expose Windows pipes by default, which are needed for Discord IPC.

Refer to [this guide](https://gist.github.com/mousebyte/af45cbecaf0028ea78d0c882c477644a#aliasing-nvim). Use `socat` and `npiperelay` to create a bridge that exposes the Windows Discord IPC pipe to WSL.

1. **Install `socat` in WSL**: `sudo apt-get install socat` (or your distribution's package manager command).
2. **Download `npiperelay.exe`**: Download `npiperelay.exe` from [https://github.com/jstarks/npiperelay/releases](https://github.com/jstarks/npiperelay/releases) and place it in a directory accessible from WSL (e.g., `/mnt/c/path/to/npiperelay.exe`).
3. **Alias `nvim` with Pipe Relay**: Add an alias to your WSL shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.) to launch `nvim` with the pipe relay setup. Example alias:
    ```sh
    nvim() {
        if ! pidof socat > /dev/null 2>&1; then
            [ -e /tmp/discord-ipc-0 ] && rm -f /tmp/discord-ipc-0
            socat UNIX-LISTEN:/tmp/discord-ipc-0,fork \
                EXEC:\"/mnt/c/path/to/npiperelay.exe //./pipe/discord-ipc-0\" &
        fi

        if [ $# -eq 0 ]; then
            command nvim
        else
            command nvim "$@"
        fi
    }
    ```

    > Adjust the path to `npiperelay.exe` in the alias if needed.
    > Always launch Neovim using the `nvim` alias you defined in WSL.

### 🖥️ Remote Server (SSH)

Use SSH port forwarding to tunnel the Discord IPC socket over SSH. Follow the guide in this article: [https://carlosbecker.com/posts/discord-rpc-ssh/](https://carlosbecker.com/posts/discord-rpc-ssh/)

## ❓ Still Having Issues?

If your problem is not listed here or the solutions don't work, please:

1.  **Check the FAQ**: See if your issue is listed in the [FAQ](./FAQ.md).
2.  **Search Existing Issues**: Check the [existing issues](https://github.com/vyfor/cord.nvim/issues) on the Cord GitHub repository to see if your issue has already been reported.
3.  **Open a New Issue**: If you think your issue is a bug in Cord, [open a new issue](https://github.com/vyfor/cord.nvim/issues/new/choose) on GitHub. Be sure to include a clear title and description, along with as much relevant information as possible. If you're unsure if it's a bug, you can always clarify in Discussions or Discord.

We're here to help! Providing detailed information in your issue will help us diagnose and resolve your problem more quickly.