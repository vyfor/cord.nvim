<div align="center">
  <h1>🧩 <strong>Cord</strong> – Tailor Your Presence Like Never Before</h1>
  <div>
    <a href="https://github.com/vyfor/cord.nvim/stargazers"><img src="https://img.shields.io/github/stars/vyfor/cord.nvim?style=for-the-badge&color=8281f3&labelColor=242529" alt="Stargazers"></a>
    <a href="https://neovim.io/"><img src="https://img.shields.io/badge/Neovim-%20%3E%3D%200.6.0-ffffff?style=for-the-badge&logo=neovim&color=8281f3&labelColor=242529&logoColor=8281f3" alt="Neovim Logo"></a>
    <a href="https://github.com/vyfor/cord.nvim/forks"><img src="https://img.shields.io/github/forks/vyfor/cord.nvim?style=for-the-badge&color=8281f3&labelColor=242529" alt="Forks"></a>
  </div>
  <br/>
  <img src="https://github.com/user-attachments/assets/8e684058-f3ea-4010-817e-529b47730abb" alt="Cord Logo" width="200px">
  <h3>🚀 The most extensible Discord Rich Presence plugin for Neovim, powered by Rust.
  </h3>
  <img src="https://github.com/user-attachments/assets/df73221e-565b-49e5-9dad-1c60aed6f9c3" alt="Cord Banner">
</div>

## 📚 Table of Contents
- [💎 Features](#-features)
- [📦 Installation](#-installation)
- [🎨 Themes](#-themes)
- [📖 Documentation](#-documentation)
- [🤝 Contributing](#-contributing)
- [❓ FAQ](#-faq)

## 💎 Features  
- 🌐 **Client-Server Design** — Handles multiple Neovim instances with a single connection to Discord.
- ⚡ **Performance in Mind** — Lightweight, dependency-free, with blazingly-fast startup.
- 🚀 **Event-Driven Architecture** — Instant presence updates with zero delays.  
- 🎨 **Customizable Templates** — Dynamic string templates with custom variables.
- 🔧 **Unmatched Configurability** — Function-based configuration for infinite customization possibilities.
- 🧠 **Automated State Handling** — Automatically manages activities across all instances.
- 💤 **Smart Idle Detection** — Identifies idle sessions and switches to the most recent non-idle session.
- 🛠️ **Built-in Git Integration** — Detects repositories and workspaces based on VCS files without relying on command-line tools.
- 🌍 **Cross-Platform** — Supports Windows, Linux (Flatpak/Snap), macOS, and BSD.
- 🌸 **Rich Icon Collection** — Features over 70 uniquely designed icons.

## 📦 Installation  

### Considerations
Cord requires the server executables to be present. To get it, you can either:
- **Fetch from GitHub**: By invoking `:Cord fetch` (async, recommended)
  - Requires **[`curl`](https://curl.se)**
- **Build and install from crates.io**: By invoking `:Cord build` (async)
  - Requires **[`Rust`](https://www.rust-lang.org/tools/install) >= 1.85.0 nightly**
- **Build from source**: By invoking `cargo b --release`, Cord will automatically move the executable.
  - Requires **[`Rust`](https://www.rust-lang.org/tools/install) >= 1.85.0 nightly**
- **Download from GitHub**: Get latest release from https://github.com/vyfor/cord.nvim/releases/latest, rename it to cord[.exe] and place it under `nvim-data-dir/cord/bin`

### Installation
<details>
<summary>Using lazy.nvim</summary>

```lua
{
  'vyfor/cord.nvim',
  branch = 'client-server',
  build = ':Cord fetch',
  opts = {}, -- calls require('cord').setup()
}
```

</details>

<details>
<summary>Using packer.nvim</summary>

```lua
use {
  'vyfor/cord.nvim',
  branch = 'client-server',
  run = ':Cord fetch',
  config = function()
    require('cord').setup()
  end
}
```

</details>

<details>
<summary>Using Vim packages</summary>

**Unix:**
```bash
git clone https://github.com/vyfor/cord.nvim.git ~/.local/share/nvim/site/pack/plugins/start/cord.nvim
```

**Windows:**
```powershell
git clone https://github.com/vyfor/cord.nvim.git $LOCALAPPDATA/nvim-data/site/pack/plugins/start/cord.nvim
```

Then call the following function somewhere in your configuration:
```lua
require('cord').setup()
```

Invoke `:Cord fetch` to whenever the plugin is updated.

</details>

<details>
<summary>Other</summary>

Make sure you call the following function somewhere in your configuration:
```lua
require('cord').setup()
```

Invoke `:Cord fetch` to whenever the plugin is updated.

</details>

## 🎨 Themes  
Cord features over 70 beautifully designed icons for languages and components with distinct themes, with more to come!

👉 [**Explore the Showcase**](https://github.com/vyfor/icons#showcase)  

## 📖 Documentation  
- [**Configuration Guide**](wiki/CONFIGURATION.md): Everything you need to customize Cord.  
- [**Examples**](wiki/EXAMPLES.md): Creative ways to customize your Discord presence.
- [**Migration Guide**](wiki/MIGRATION.md): Smooth migration from Cord v1.
- [**Wiki**](https://github.com/vyfor/cord.nvim/wiki): Examples, best practices, and FAQs. (Coming soon)

## 🤝 Contributing  
We welcome contributions to make Cord even better!
- Check out our [**Contribution Guidelines**](.github/CONTRIBUTING.md).  

## ❓ FAQ  
Have questions or issues?  
- [**FAQ**](https://github.com/vyfor/cord.nvim/wiki/FAQ)  
- [**Troubleshooting Guide**](https://github.com/vyfor/cord.nvim/wiki/Troubleshooting)  

---

<div align="center">  
  <p>💬 Questions? Reach me out on Discord: <a href="https://discord.com/users/446729269872427018"><strong>vyfor</strong></a></p>  
</div>
