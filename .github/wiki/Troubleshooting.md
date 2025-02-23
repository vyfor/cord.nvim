# 🔧 Troubleshooting

Having trouble with Cord? This guide provides solutions to common problems and scenarios you might encounter.

## 🛠️ General Troubleshooting Steps

If you're experiencing issues, try these general steps first:

1.  **Check Plugin Loading**: Ensure `cord.nvim` is correctly loaded. Try running a Cord user command like `:Cord status`. If it's not recognized, the plugin may not be installed or loaded correctly.
2.  **Restart Neovim**: Sometimes a simple restart of Neovim can resolve temporary issues.
3.  **Update Cord**: Make sure you are using the latest version of Cord, as well as the server executable, which can be updated using `:Cord update`.
4.  **Check Discord Activity Privacy**: Verify your Discord [Activity Privacy settings](https://github.com/vyfor/cord.nvim/assets/92883017/c0c8c410-e90e-425e-bf10-8b59f04f15ce) are enabled to allow Rich Presence to be displayed.
5.  **Enable Logging**: Set `log_level = vim.log.levels.TRACE` in your `cord.setup()` configuration. Then check `:messages` for detailed logs that might indicate the problem. Remember to revert to a less verbose log level (e.g., `vim.log.levels.WARN` or `vim.log.levels.OFF`) after troubleshooting.
6.  **Run `:checkhealth cord`**:  This command performs health checks and can identify common configuration or environment issues.
7.  **Ensure Discord IPC Pipe Exists**: Check if the Discord IPC pipe exists on your system:
    - **Windows (PowerShell):** `Test-Path \\.\pipe\discord-ipc-0`
    - **Unix (Bash):** `find /tmp /var/run /run -type s -name 'discord-ipc-*' 2>/dev/null`
    If the command returns nothing or "False", the pipe may not be available.

If these general steps don't resolve your issue, look for specific problems and solutions below.

## 🚫 Rich Presence Not Showing in Discord

If Cord is loaded but your Rich Presence is not appearing in Discord, try these solutions:

- **Web/Custom Discord Client**:
    - **Problem**: The web version of Discord and some custom Discord clients may not expose the necessary IPC (Inter-Process Communication) pipe for Rich Presence.
    - **Solution**:
        - **Use Official Discord Client**: Use the official Discord desktop application (download from [https://discord.com/download](https://discord.com/download)).
        - **arrpc Bridge (for Web Discord)**: If you must use web Discord, you can try using [arrpc](https://github.com/OpenAsar/arrpc) to create a bridge. Follow arrpc's instructions to set up the bridge between your browser and Cord.

## 🎛️ No Buttons Showing in Rich Presence

- **Problem**: You've configured buttons in your `cord.setup()`, but they are not visible in Discord.
- **Solution**:
    - **Discord UI Bug**: There's a known client-side bug in Discord related to user profiles. *You may not be able to see buttons on your own Rich Presence*.
    - **Verify with Others**: Ask a friend or someone else to check your Discord profile to see if they can see the buttons on your Rich Presence. Buttons *are* visible to others even if you can't see them yourself.
    - **Voice Channel Hover**:  Another workaround to check buttons yourself is to join a voice channel in Discord and hover over your name in the voice channel list. Buttons appear in the hover tooltip.

## ⏱️ Rich Presence Timer Stuck at 00:00

- **Problem**: The elapsed time timer in your Rich Presence is stuck at "00:00" and not updating.
- **Solution**:
    - **System Date & Time**: This issue is almost always caused by an incorrect system date, time, or timezone setting on your computer.
    - **Sync System Clock**: **Ensure your system date, time, and timezone are set correctly and synchronized** with a reliable time source (e.g., using NTP - Network Time Protocol).  Operating systems usually have settings to automatically sync time.

## 💻 Running Cord in Specific Environments

### 🌐 Using Discord in a Browser

- **Challenge**: Discord running in a web browser does not expose the IPC pipe that Cord needs to communicate with it directly.
- **Solution**: **arrpc Bridge**: Use the [arrpc](https://github.com/OpenAsar/arrpc) bridge as mentioned earlier. arrpc creates a communication bridge that allows Cord to send presence updates to web-based Discord. Follow arrpc's setup instructions carefully.

### 🐧 Running Inside WSL (Windows Subsystem for Linux)

- **Challenge**: WSL (Windows Subsystem for Linux) does not directly expose Windows pipes by default, which are needed for Discord IPC.
- **Solution**: **socat and npiperelay**: Refer to [this guide](https://gist.github.com/mousebyte/af45cbecaf0028ea78d0c882c477644a#aliasing-nvim). Use `socat` and `npiperelay` to create a bridge that exposes the Windows Discord IPC pipe to WSL.
    1.  **Install `socat` in WSL**: `sudo apt-get install socat` (or your distribution's package manager command).
    2.  **Download `npiperelay.exe`**: Download `npiperelay.exe` from [https://github.com/jstarks/npiperelay/releases](https://github.com/jstarks/npiperelay/releases) and place it in a directory accessible from WSL (e.g., `/mnt/c/path/to/npiperelay.exe`).
    3.  **Alias `nvim` with Pipe Relay**: Add an alias to your WSL shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.) to launch `nvim` with the pipe relay setup. Example alias:

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

        **Adjust the path to `npiperelay.exe` in the alias if needed.**
    4.  **Use `nvim` Alias**:  Always launch Neovim using the `nvim` alias you defined in WSL.

### 🖥️ Remote Server (SSH)

- **Challenge**: Running Neovim on a remote server via SSH and displaying Rich Presence on your local Discord client.
- **Solution**: **SSH Port Forwarding**: Use SSH port forwarding to tunnel the Discord IPC socket over SSH. Follow the guide in this article: [https://carlosbecker.com/posts/discord-rpc-ssh/](https://carlosbecker.com/posts/discord-rpc-ssh/)

## ❓ Still Having Issues?

If your problem is not listed here or the solutions don't work, please:

1.  **Search Existing Issues**: Check the [existing issues](https://github.com/vyfor/cord.nvim/issues) on the Cord GitHub repository to see if your issue has already been reported.
2.  **Open a New Issue**: If you can't find a solution, [open a new issue](https://github.com/vyfor/cord.nvim/issues/new/choose) on GitHub. Be sure to include a clear title and description, along with as much relevant information as possible.

We're here to help! Providing detailed information in your issue will help us diagnose and resolve your problem more quickly.