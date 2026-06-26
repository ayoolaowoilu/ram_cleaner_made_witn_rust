use std::io::{stdin, stdout, Write};
use std::path::Path;

mod resources;
mod utils;

use crate::resources::paths::CACHE_PATHS;
use crate::utils::scan::{collect_cache_files, get_user_cache_dirs, resolve_paths, scan_dir};
use crate::utils::clean::{confirm_wipe, format_bytes, wipe_cache_dir};

fn main() {
    print_header();

    loop {
        print_menu();
        let input = get_input("Select Your option [1-4]: ");

        match input.as_str() {
            "1" => auto_scan_cache_files(false), // false = no wipe, just scan
            "2" => scan_static_paths(false),
            "3" => wipe_menu(),
            "4" => {
                println!("\n\x1B[32mGoodbye!\x1B[0m");
                break;
            }
            _ => println!("\n\x1B[31mInvalid option. Please enter 1-4.\x1B[0m\n"),
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
    println!("[1] Scan cache files (safe, read-only)");
    println!("[2] Scan static paths (safe, read-only)");
    println!("[3] Wipe / Clean cache files");
    println!("[4] Exit.\n");
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



fn scan_static_paths(wipe: bool) {
    println!("\n\x1B[36m=== Scanning Static Cache Paths ===\x1B[0m");

    for entry in &CACHE_PATHS {
        let path = resolve_paths(entry.path);
        if Path::new(&path).exists() {
            scan_dir(&path, entry.name);

            if wipe {
                let cache_files = collect_cache_files(&path);
                if !cache_files.is_empty() && confirm_wipe(&format!("Delete {} cache files from {}?", cache_files.len(), entry.name)) {
                    let (count, bytes) = wipe_cache_dir(Path::new(&path), false);
                    println!("\n\x1B[32mDeleted {} files, freed {}\x1B[0m", count, format_bytes(bytes));
                }
            }
        } else {
            println!("\n \x1B[33m[SKIP]\x1B[0m Path not found: {} ({})", entry.name, path);
        }
    }
}

fn auto_scan_cache_files(wipe: bool) {
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

            if wipe {
                let cache_files = collect_cache_files(&dir);
                if !cache_files.is_empty() && confirm_wipe(&format!("Delete cache files from {}?", name)) {
                    let (count, bytes) = wipe_cache_dir(Path::new(&dir), false);
                    println!("\n\x1B[32mDeleted {} files, freed {}\x1B[0m", count, format_bytes(bytes));
                }
            }
        } else {
            println!("\n \x1B[33m[SKIP]\x1B[0m Directory not found: {}", dir);
        }
    }
}

// ============== WIPE MENU ==============

fn wipe_menu() {
    println!("\n\x1B[36m=== Wipe Cache Files ===\x1B[0m");
    println!("[1] Preview what will be deleted (dry-run)");
    println!("[2] Wipe auto-detected cache directories");
    println!("[3] Wipe static cache paths");
    println!("[4] Back to main menu");

    let choice = get_input("\nSelect option: ");

    match choice.as_str() {
        "1" => dry_run_preview(),
        "2" => auto_scan_cache_files(true), // true = enable wipe
        "3" => scan_static_paths(true),
        "4" => return,
        _ => println!("\x1B[31mInvalid choice\x1B[0m"),
    }
}

fn dry_run_preview() {
    println!("\n\x1B[36m=== Dry Run Preview ===\x1B[0m");

    let dirs = get_user_cache_dirs();
    for dir in dirs {
        if Path::new(&dir).exists() {
            let (count, bytes) = wipe_cache_dir(Path::new(&dir), true); // true = dry run
            if count > 0 {
                println!("\n  {}: {} files, {}", dir, count, format_bytes(bytes));
            }
        }
    }

    println!("\n\x1B[90m(Dry run complete - no files were deleted)\x1B[0m");
}