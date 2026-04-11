<div align="center">

# ⌨️ Aaahhh 

**A lightning-fast, ultra-lightweight, native mechanical keyboard sound simulator written in Rust.**

[![Rust](https://img.shields.io/badge/rust-1.74%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey.svg)]()

Experience the satisfying auditory feedback of premium mechanical switches on *any* keyboard you use. This project operates seamlessly as a true system daemon launch it once and it detaches from your terminal entirely, letting you work in peace with beautiful acoustics in the background!

</div>

---

## 📑 Table of Contents
- [✨ Features](#-features)
- [📦 Installation](#-installation)
- [🚀 Usage](#-usage)
- [🏗️ Project Architecture](#️-project-architecture)
- [🛠️ Diagnostics / Debugging](#️-diagnostics--debugging)
- [👤 Author](#-author)

---

## ✨ Features

- **Blazing Fast**: Built entirely in Rust. Minimal latency, zero electron-bloat, bare-minimum memory footprint.
- **Premium Switch Profiles**: Choose from pristine audio profiles:
  - 🔴 `Cherry MX Red`
  - ⚫ `Gateron Black`
  - 🐼 `Holy Panda`
- **True Background Daemon**: Built-in support to detach safely from your terminal. Launch it, close your window, and enjoy the sounds.
- **Smart Key Handling**: Implements a dedicated HashSet key-tracker to prevent annoying repetitive audio spam during long, continuous key-holds.
- **Cross-Platform**: Natively hooks into global key events on macOS, Windows, and Linux via `rdev`.

---

## 📦 Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed on your machine.

1. Clone this repository:
```bash
git clone https://github.com/subhdotsol/aaahhh.git
cd aaahhh
```

2. Build and install via Cargo:
```bash
cargo install --path . --force
```

*(This compiles the tool from source with optimizations and places the `aaahhh` binary securely into your global Cargo bin directory!)*

---

## 🚀 Usage

Aaahhh is designed with an extremely simple CLI designed to get out of your way.

### Start the Simulator

```bash
aaahhh start
```
This drops you into a quick, interactive dropdown menu to let you select your desired mechanical switch. 

> **Note:** *If this is your first time using a specific switch, Aaahhh will automatically fetch and unpack the exact sound profiles required into a hidden `~/.aaahhh` directory in your home path.*

Once selected, Aaahhh instantly spins up the audio listener natively in the background and releases your terminal line. You can safely close the window and the sounds will track globally across your OS!

### Stop the Simulator

```bash
aaahhh stop
```
There's no need to reach for your task manager! Aaahhh utilizes process-ID (PID) targeting. Executing `aaahhh stop` instantly searches for your active background daemon and safely shuts it down from any terminal window.

---

## 🏗️ Project Architecture

The codebase operates on a decoupled structure, separating CLI lifecycle, core configurations, and audio engines:

```text
src/
├── main.rs            # Entrypoint wrapper and overarching CLI router
├── cli/               # Command-line workflows
│   ├── mod.rs
│   ├── start.rs       # Handles interactive menu & detached process firing
│   ├── daemon.rs      # Background worker logic & PID registration
│   └── stop.rs        # Sysinfo-powered background termination
├── audio/             # Sound execution logic
│   ├── mod.rs
│   ├── player.rs      # Native thread parking and smart key-hold trackers
│   └── sounds.rs      # Sound definition enum mapping & data urls
└── core/              # Foundation utilities & memory
    ├── mod.rs
    ├── config.rs      # Audio layout JSON parsing definitions
    ├── constants.rs   # Persistent directories & Keymaps
    ├── errors.rs      # Global `EchoErrors` implementation wrapper
    └── utils.rs       # External payload extraction and generic I/O
```

---

## 🛠️ Diagnostics / Debugging

If you are developing or experiencing hook-failure issues, you can append the built-in debug flag when spawning:

```bash
aaahhh start --debug
```

This forces the tool to emit verbose standard logs (e.g., directory allocation, hook recognition times, missing config errors).

> **Operating System Warning:** *Depending on your specific operating system (especially on macOS), you may need to ensure your active terminal emulator has system-level **Accessibility API** permissions enabled in settings to successfully capture global keystrokes!*

---

## 👤 Author

**Subhajit chaudhury**  
GitHub: [@subhdotsol](https://github.com/subhdotsol)
<!-- Doc verification -->
