<div align="center">
  <h1>🧩 <strong>Cord</strong> – Tailor Your Presence Like Never Before</h1>
  <div>
    <a href="https://github.com/vyfor/cord.nvim/stargazers"><img src="https://img.shields.io/github/stars/vyfor/cord.nvim?style=for-the-badge" alt="Stargazers"></a>
    <a href="https://github.com/vyfor/cord.nvim/blob/master/LICENSE"><img src="https://img.shields.io/github/license/vyfor/cord.nvim?style=for-the-badge" alt="Apache-2.0 License"></a>
    <a href="https://github.com/vyfor/cord.nvim/forks"><img src="https://img.shields.io/github/forks/vyfor/cord.nvim?style=for-the-badge" alt="Forks"></a>
  </div>
  <h3>🚀 The most extensible Discord Rich Presence plugin for Neovim, powered by Rust.
  </h3>
  <img src="https://github.com/user-attachments/assets/df73221e-565b-49e5-9dad-1c60aed6f9c3" alt="Cord Banner">
</div>

## 📚 Table of Contents
- [✨ Key Features](#-key-features)
- [🔌 Requirements](#-requirements)
- [📦 Installation](#-installation)
- [🎨 Themes](#-themes)
- [📖 Documentation](#-documentation)
- [🤝 Contributing](#-contributing)
- [❓ FAQ](#-faq)

## ✨ Key Features  
- 🌐 **Client-Server Design** — Handles multiple Neovim instances with a single connection to Discord.
- ⚡ **Performance in Mind** — Lightweight, dependency-free, with blazingly-fast startup.
- 🚀 **Event-Driven Architecture** — Instant presence updates with zero delays.  
- 🎨 **Customizable Templates** — Dynamic string templates with custom variables.
- 🔧 **Unmatched Configurability** — Function-based configuration for infinite customization possibilities.
- 🧠 **Automated State Handling** — Automatically manages activities across all instances.
- 💤 **Smart Idle Detection** — Identifies idle sessions and switches to the most recent non-idle session.
- 🛠️ **Built-in Git Integration** — Detects repositories and workspaces based on VCS.
- 🌍 **Cross-Platform** — Supports Windows, Linux (Flatpak/Snap), macOS, and BSD.

## 🔌 Requirements  
- **Neovim >= 0.6.0** 
- **[Rust](https://www.rust-lang.org/tools/install) >= 1.85.0 nightly**

## 📦 Installation  

<details>
<summary>Using lazy.nvim</summary>

```lua
{
  'vyfor/cord.nvim',
  build = 'cargo build --release',
  opts = {}, -- calls require('cord').setup()
}
```

</details>

<details>
<summary>Using packer.nvim</summary>

```lua
use {
  'vyfor/cord.nvim',
  run = 'cargo build --release',
  config = function()
    require('cord').setup()
  end
}
```

</details>

## 🎨 Themes  
Cord features over 70 beautifully designed icons for languages and components with distinct themes.

👉 [**Explore the Showcase**](https://github.com/vyfor/icons#showcase)  

## 📖 Documentation  
- [**Configuration Guide**](wiki/CONFIGURATION.md): Everything you need to customize Cord.  
- [**Migration Guide**](wiki/MIGRATION.md): Smooth migration from Cord v1.  
- [**Wiki**](https://github.com/vyfor/cord.nvim/wiki): Examples, best practices, and FAQs.

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
