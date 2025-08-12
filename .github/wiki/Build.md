# 🏗️ Build From Source

> [!NOTE]
> Make sure you have **[Rust](https://www.rust-lang.org/tools/install)** >= 1.85.0 (nightly) installed.

1. Clone the repository and move into the directory
   ```bash
   git clone https://github.com/vyfor/cord.nvim
   ```
2. Build the server binary and place it under the Neovim data directory
   **Linux:**
   ```bash
   cargo install --path . --root ~/.local/share/nvim/cord --force
   ```

   **Windows (PowerShell):**
   ```powershell
   cargo install --path . --root $env:LOCALAPPDATA/nvim-data/cord --force
   ```