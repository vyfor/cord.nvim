# 💻 Special Environments

## 🐧 Running inside WSL

WSL doesn't expose existing Windows named pipes by default, which Cord needs. To work around that, use `socat` and `npiperelay`.
This method is based on [this gist](https://gist.github.com/mousebyte/af45cbecaf0028ea78d0c882c477644a#aliasing-nvim).

1. **Install `socat`** in WSL: `sudo apt install socat`
2. **Get `npiperelay.exe`** from [here](https://github.com/jstarks/npiperelay/releases) and place it in a path accessible from WSL, preferably add it to PATH.
3. **Add this `nvim` alias** in your `.bashrc`, `.zshrc`, etc.:

   ```sh
   nvim() {
        if ! pidof socat > /dev/null 2>&1; then
            [ -e /tmp/discord-ipc-0 ] && rm -f /tmp/discord-ipc-0
            socat UNIX-LISTEN:/tmp/discord-ipc-0,fork \
                EXEC:"npiperelay.exe //./pipe/discord-ipc-0" 2>/dev/null &
        fi

        command nvim "$@"
   }
   ```

   > Do note that you must either add the path to npiperelay to your Windows PATH, or specify an absolute path to it inside `EXEC`.
   > Always launch Neovim using this alias in WSL.

## 🖥️ Remote Server (SSH)

You can forward the Discord IPC socket over SSH. [This
](https://carlosbecker.com/posts/discord-rpc-ssh/) article explains how.

## 🌐 Using Discord in a Browser

Use [arrpc](https://github.com/OpenAsar/arrpc) as a bridge. Follow its instructions closely at your own risk.

> [!IMPORTANT]
> arrpc has been left unmaintained for quite some time.

## 🧪 Custom Discord Clients

Cord can work with custom clients, although we do not endorse them, and cannot guarantee that they will work. The main issue is that custom clients often cannot/do not expose the IPC pipe at the same path as the official client, so you might need to create a symlink to make it work.

You can also override the defaults by setting the `advanced.discord.pipe_paths` field to a list of absolute paths to use when connecting to Discord.
