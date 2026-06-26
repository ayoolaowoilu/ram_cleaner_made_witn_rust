use std::io::{stdin, stdout, Write};
use std::path::Path;

mod resources;
mod utils;

use crate::resources::paths::CACHE_PATHS;
use crate::utils::scan::{get_user_cache_dirs, resolve_paths, scan_dir};

fn main() {
    print_header();

    loop {
        print_menu();
        let input = get_input("Select Your option [1, 2 or 3]: ");

        match input.as_str() {
            "1" => auto_scan_cache_files(),
            "2" => scan_static_paths(),
            "3" => {
                println!("\n\x1B[32mGoodbye!\x1B[0m");
                break;
            }
            _ => println!("\n\x1B[31mInvalid option. Please enter 1, 2, or 3.\x1B[0m\n"),
        }

        println!("\n\x1B[90mPress Enter to continue...\x1B[0m");
        let _ = get_input("");
        clear_screen();
    }
}

fn print_header() {
    println!("\n\x1B[36m======== Welcome to ram_cleaner (Rust) ========\x1B[0m\n");
}

fn print_menu() {
    println!("Select what you want to do today.\n");
    println!("[1] Auto scan and clean cache files on your computer.");
    println!("[2] Manual cleaning and scanning of cache (our static paths) on your system.");
    println!("[3] Exit.\n");
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    input.trim().to_string()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn scan_static_paths() {
    println!("\n\x1B[36m=== Scanning Static Cache Paths ===\x1B[0m");

    for entry in &CACHE_PATHS {
        let path = resolve_paths(entry.path);
        if Path::new(&path).exists() {
            scan_dir(&path, entry.name);
        } else {
            println!(
                "\n \x1B[33m[SKIP]\x1B[0m Path not found: {} ({})",
                entry.name, path
            );
        }
    }
}

fn auto_scan_cache_files() {
    println!("\n\x1B[36m=== Scanning User Cache Directories ===\x1B[0m");

    let dirs = get_user_cache_dirs();
    if dirs.is_empty() {
        println!("\n \x1B[33m[WARNING]\x1B[0m No cache directories found.");
        return;
    }

    for dir in dirs {
        let name = Path::new(&dir)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if Path::new(&dir).exists() {
            scan_dir(&dir, name);
        } else {
            println!("\n \x1B[33m[SKIP]\x1B[0m Directory not found: {}", dir);
        }
    }
}