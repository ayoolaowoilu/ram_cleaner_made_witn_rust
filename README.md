ram_cleaner
===========

A fast, safe, and lightweight cache cleaning utility written in Rust. Scan and wipe temporary/cache files from your system to free up disk space.

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

Cross-Platform Support
~~~~~~~~~~~~~~~~~~~~~~
- Windows: Fully supported
- macOS: Fully supported
- Linux: Fully supported

Installation
------------

From Source
~~~~~~~~~~~
    git clone https://github.com/ayoolaowoilu/ram_cleaner_made_witn_rust.git
    cd ram_cleaner
    cargo build --release
    ./target/release/ram_cleaner

Requirements
~~~~~~~~~~~~
- Rust 1.70+

Usage
-----

Interactive Menu
~~~~~~~~~~~~~~~~
Launch the app and select an option from the menu:

    $ ram_cleaner

    ======== Welcome to ram_cleaner (Rust) ========

    Select what you want to do today.

    [1] Scan cache files (safe, read-only)
    [2] Scan static paths (safe, read-only)
    [3] Wipe / Clean cache files
    [4] Exit.

    Select Your option [1-4]:

Option 1: Auto Scan
~~~~~~~~~~~~~~~~~~~
Scans all user-accessible cache directories:
- Windows: %LOCALAPPDATA%\Temp, %APPDATA%, etc.
- macOS: ~/Library/Caches
- Linux: ~/.cache, /tmp

Option 2: Static Paths
~~~~~~~~~~~~~~~~~~~~~
Scans predefined application cache locations:
- Chrome Cache
- Firefox Cache
- System Temp
- Windows Explorer Thumbnails

Option 3: Wipe
~~~~~~~~~~~~~~
    [1] Preview what will be deleted (dry-run)
    [2] Wipe auto-detected cache directories
    [3] Wipe static cache paths
    [4] Back to main menu

Dry Run shows exactly what would be deleted -- no files are touched.

Safety
------

What We Protect
~~~~~~~~~~~~~~~
ram_cleaner includes multiple safety layers to prevent accidental data loss:

- Path Validation: Only deletes from known cache/temp directories
- System Directory Blocklist: Blocks System32, Program Files, SysWOW64, Boot, Drivers
- User Confirmation: Every wipe requires typing y or yes
- Dry Run Mode: Preview deletions before committing
- File Pattern Matching: Only targets .cache, .tmp, .temp, thumbs.db, etc.

What Never Gets Deleted
~~~~~~~~~~~~~~~~~~~~~~~
- System files (.sys, .dll)
- User documents, downloads, pictures, desktop files
- Registry files
- Boot files

Project Structure
-----------------

    ram_cleaner/
    |-- Cargo.toml
    |-- README.md
    |-- src/
        |-- main.rs              # Entry point, menu loop
        |-- resources/
        |   |-- paths.rs         # Static cache path definitions
        |-- utils/
            |-- scan.rs          # Directory scanning logic
            |-- clean.rs         # File deletion & safety checks

Key Modules
~~~~~~~~~~~
- main.rs: CLI menu, user input handling
- resources/paths.rs: Hardcoded application cache paths
- utils/scan.rs: scan_dir(), collect_cache_files(), get_user_cache_dirs()
- utils/clean.rs: wipe_cache_dir(), is_safe_to_delete(), confirm_wipe()

Screenshots
-----------

Main Menu
~~~~~~~~~
    ======== Welcome to ram_cleaner (Rust) ========

    Select what you want to do today.

    [1] Scan cache files (safe, read-only)
    [2] Scan static paths (safe, read-only)
    [3] Wipe / Clean cache files
    [4] Exit.

    Select Your option [1-4]: 1

Scan Results
~~~~~~~~~~~~
    === Scanning User Cache Directories ===

     Temp
     Files : 15234
     Folders : 892
     Size : 4.20GB
     Cache Files : 3421
     Cache Size : 1.80GB

Dry Run Preview
~~~~~~~~~~~~~~~
    === Dry Run Preview ===

      C:\Users\You\AppData\Local\Temp: 1,247 files, 856.32 MB
      C:\Users\You\AppData\Local\Microsoft\Windows\Explorer: 89 files, 12.40 MB

    (Dry run complete - no files were deleted)

Contributing
------------

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request

Development
~~~~~~~~~~~
    cargo run          # Run in development mode
    cargo test         # Run tests
    cargo fmt          # Format code
    cargo clippy       # Lint

License
-------

This project is licensed under the MIT License -- see the LICENSE file for details.

Acknowledgments
---------------

- Built with Rust (https://www.rust-lang.org/)
- Directory walking powered by walkdir (https://crates.io/crates/walkdir)
