ram_cleaner
===========

<p align="center">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-green.svg?style=for-the-badge" alt="Platform">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/ayoolaowoilu/ram_cleaner_made_witn_rust/main/assets/logo.png" width="120" alt="ram_cleaner logo">
</p>

<h1 align="center">ram_cleaner</h1>

<p align="center">
  <b>A fast, safe, and lightweight cache cleaning utility written in Rust.</b><br>
  Scan and wipe temporary/cache files from your system to free up disk space.
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/ayoolaowoilu/ram_cleaner_made_witn_rustmain/assets/demo.gif" width="600" alt="Demo">
</p>

---

Table of Contents
-----------------

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Safety](#safety)
- [Project Structure](#project-structure)
- [Screenshots](#screenshots)
- [Contributing](#contributing)
- [License](#license)

---

Features
--------

Scan Modes
~~~~~~~~~~
- Auto Scan -- Detects and scans all user-level cache directories automatically
- Static Scan -- Scans predefined application paths (Chrome, Firefox, Temp, etc.)
- Dry Run -- Preview what will be deleted without touching any files

Wipe Modes
~~~~~~~~~~
- Safe Deletion -- Built-in guards prevent deletion of system-critical files
- Confirmation Prompts -- Every wipe requires explicit user confirmation
- Recursive Cleaning -- Deep cleans nested cache folders

## Cross Platform

| Platform | Status | Icon |
|----------|--------|------|
| Windows | Fully supported | <img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white"> |
| macOS | Fully supported | <img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white"> |
| Linux | Fully supported | <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black"> |

---

Installation
------------

From Source
~~~~~~~~~~~
    git clone https://github.com/ayoolaowoilu/ram_cleaner_made_witn_rust.git
    cd ram_cleaner
    cargo build --release
    ./target/release/ram_cleaner
~~~~~~~~~~~~
## Requirements

- Rust 1.70+
- Cargo

---

Usage
-----

## Interactive Menu

Launch the app and select an option from the menu:

    $ ram_cleaner

    ======== Welcome to ram_cleaner (Rust) ========

    Select what you want to do today.

    [1] Scan cache files (safe, read-only)
    [2] Scan static paths (safe, read-only)
    [3] Wipe / Clean cache files
    [4] Exit.

    Select Your option [1-4]:

## Option 1: Auto Scan

Scans all user-accessible cache directories:

| OS | Paths Scanned |
|----|---------------|
| <img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white"> | `%LOCALAPPDATA%\Temp`, `%APPDATA%`, etc. |
| <img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white"> | `~/Library/Caches` |
| <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black"> | `~/.cache`, `/tmp` |

## Option 2: Static Paths

Scans predefined application cache locations:

| Application | Path |
|-------------|------|
| <img src="https://img.shields.io/badge/Chrome-4285F4?style=flat-square&logo=google-chrome&logoColor=white"> | Chrome Cache |
| <img src="https://img.shields.io/badge/Firefox-FF7139?style=flat-square&logo=firefox&logoColor=white"> | Firefox Cache |
| <img src="https://img.shields.io/badge/Temp-6E7681?style=flat-square"> | System Temp |
| <img src="https://img.shields.io/badge/Explorer-0078D6?style=flat-square&logo=windows&logoColor=white"> | Windows Explorer Thumbnails |

Option 3: Wipe
~~~~~~~~~~~~~~
    [1] Preview what will be deleted (dry-run)
    [2] Wipe auto-detected cache directories
    [3] Wipe static cache paths
    [4] Back to main menu

Dry Run shows exactly what would be deleted -- no files are touched.
~~~~~~~~~~~~~~~
---

Safety
------

What We Protect

ram_cleaner includes multiple safety layers to prevent accidental data loss:

| Protection | Icon | Description |
|------------|------|-------------|
| Path Validation | <img src="https://img.shields.io/badge/SAFE-2ea44f?style=flat-square"> | Only deletes from known cache/temp directories |
| System Directory Blocklist | <img src="https://img.shields.io/badge/BLOCKED-d93f0b?style=flat-square"> | Blocks System32, Program Files, SysWOW64, Boot, Drivers |
| User Confirmation | <img src="https://img.shields.io/badge/CONFIRM-fbca04?style=flat-square"> | Every wipe requires typing `y` or `yes` |
| Dry Run Mode | <img src="https://img.shields.io/badge/DRY%20RUN-6f42c1?style=flat-square"> | Preview deletions before committing |
| File Pattern Matching | <img src="https://img.shields.io/badge/FILTER-0366d6?style=flat-square"> | Only targets `.cache`, `.tmp`, `.temp`, `thumbs.db`, etc. |

## What Never Gets Deleted

- <img src="https://img.shields.io/badge/PROTECTED-d93f0b?style=flat-square"> System files (`.sys`, `.dll`)
- <img src="https://img.shields.io/badge/PROTECTED-d93f0b?style=flat-square"> User documents, downloads, pictures, desktop files
- <img src="https://img.shields.io/badge/PROTECTED-d93f0b?style=flat-square"> Registry files
- <img src="https://img.shields.io/badge/PROTECTED-d93f0b?style=flat-square"> Boot files

---

Project Structure
-----------------

    ram_cleaner/
    |-- Cargo.toml
    |-- README.md
    |-- assets/
    |   |-- logo.png
    |   |-- demo.gif
    |   |-- screenshot1.png
    |   |-- screenshot2.png
    |-- src/
        |-- main.rs              # Entry point, menu loop
        |-- resources/
        |   |-- paths.rs         # Static cache path definitions
        |-- utils/
            |-- scan.rs          # Directory scanning logic
            |-- clean.rs         # File deletion & safety checks

## Key Modules

| File | Purpose | Icon |
|------|---------|------|
| `main.rs` | CLI menu, user input handling | <img src="https://img.shields.io/badge/ENTRY-2ea44f?style=flat-square"> |
| `resources/paths.rs` | Hardcoded application cache paths | <img src="https://img.shields.io/badge/CONFIG-0366d6?style=flat-square"> |
| `utils/scan.rs` | `scan_dir()`, `collect_cache_files()`, `get_user_cache_dirs()` | <img src="https://img.shields.io/badge/SCAN-6f42c1?style=flat-square"> |
| `utils/clean.rs` | `wipe_cache_dir()`, `is_safe_to_delete()`, `confirm_wipe()` | <img src="https://img.shields.io/badge/CLEAN-d93f0b?style=flat-square"> |

---

Screenshots
-----------

## Main Menu

<p align="center">
  <img src="https://raw.githubusercontent.com/yourusername/ram_cleaner/main/assets/screenshot1.png" width="600" alt="Main Menu">
</p>

## Scan Results

<p align="center">
  <img src="https://raw.githubusercontent.com/yourusername/ram_cleaner/main/assets/screenshot2.png" width="600" alt="Scan Results">
</p>

## Dry Run Preview

    === Dry Run Preview ===

      C:\Users\You\AppData\Local\Temp: 1,247 files, 856.32 MB
      C:\Users\You\AppData\Local\Microsoft\Windows\Explorer: 89 files, 12.40 MB

    (Dry run complete - no files were deleted)

---

Contributing
------------

Contributions are welcome! Please follow these steps:

1. <img src="https://img.shields.io/badge/1-2ea44f?style=flat-square"> Fork the repository
2. <img src="https://img.shields.io/badge/2-2ea44f?style=flat-square"> Create a feature branch (`git checkout -b feature/amazing-feature`)
3. <img src="https://img.shields.io/badge/3-2ea44f?style=flat-square"> Commit your changes (`git commit -m 'Add amazing feature'`)
4. <img src="https://img.shields.io/badge/4-2ea44f?style=flat-square"> Push to the branch (`git push origin feature/amazing-feature`)
5. <img src="https://img.shields.io/badge/5-2ea44f?style=flat-square"> Open a Pull Request

## Development

    cargo run          # Run in development mode
    cargo test         # Run tests
    cargo fmt          # Format code
    cargo clippy       # Lint

---

License
-------

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge" alt="License">
</p>

This project is licensed under the MIT License -- see the [LICENSE](LICENSE) file for details.

---

Acknowledgments
---------------

- <img src="https://img.shields.io/badge/Built%20with-Rust-000000?style=flat-square&logo=rust&logoColor=white"> Built with [Rust](https://www.rust-lang.org/)
- <img src="https://img.shields.io/badge/Powered%20by-walkdir-2ea44f?style=flat-square"> Directory walking powered by [walkdir](https://crates.io/crates/walkdir)
- <img src="https://img.shields.io/badge/Icons-Shields.io-6f42c1?style=flat-square"> Badges by [Shields.io](https://shields.io/)

---

<p align="center">
  <img src="https://img.shields.io/badge/Star%20%E2%AD%90-if%20you%20like%20it!-ff69b4?style=for-the-badge" alt="Star">
</p>
