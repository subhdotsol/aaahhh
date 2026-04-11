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
