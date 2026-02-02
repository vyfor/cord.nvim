# 🏗️ Build From Source

> [!NOTE]
> Make sure you have **[Rust](https://www.rust-lang.org/tools/install)** >= 1.89.0 installed.

1. Clone the repository and move into the directory
   ```bash
   git clone https://github.com/vyfor/cord.nvim
   ```
2. Build the server binary using `:Cord update build` which will place the built binary under the Neovim data directory

> [!IMPORTANT]
> If you are building from source, it is recommended to either set `advanced.server.update` mode to `build`/`install`, or disable automatic updates altogether (`advanced.server.auto_update = false`).