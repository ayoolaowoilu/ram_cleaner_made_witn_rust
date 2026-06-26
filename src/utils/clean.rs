use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};


pub fn wipe_cache_files(files: &[PathBuf], dry_run: bool) -> (usize, u64) {
    let mut deleted_count: usize = 0;
    let mut freed_bytes: u64 = 0;

    for file in files {
        if !file.exists() {
            continue;
        }

        // Safety: never delete outside known cache dirs
        if !is_safe_to_delete(file) {
            println!("  \x1B[33m[SKIP UNSAFE]\x1B[0m {:?}", file);
            continue;
        }

        if dry_run {
            println!("  \x1B[90m[DRY-RUN] Would delete\x1B[0m {:?}", file);
        } else {
            match fs::remove_file(file) {
                Ok(_) => {
                    if let Ok(meta) = fs::metadata(file) {
                        freed_bytes += meta.len();
                    }
                    deleted_count += 1;
                    println!("  \x1B[32m[DELETED]\x1B[0m {:?}", file);
                }
                Err(e) => {
                    println!("  \x1B[31m[FAILED]\x1B[0m {:?} - {}", file, e);
                }
            }
        }
    }

    (deleted_count, freed_bytes)
}

/// Recursively wipe a cache directory (keeps the dir, empties contents)
pub fn wipe_cache_dir(dir: &Path, dry_run: bool) -> (usize, u64) {
    let mut deleted_count: usize = 0;
    let mut freed_bytes: u64 = 0;

    if !dir.exists() {
        return (0, 0);
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            println!("  \x1B[31m[ERROR]\x1B[0m Cannot read {:?}: {}", dir, e);
            return (0, 0);
        }
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if !is_safe_to_delete(&path) {
            println!("  \x1B[33m[SKIP UNSAFE]\x1B[0m {:?}", path);
            continue;
        }

        if dry_run {
            if path.is_file() {
                println!("  \x1B[90m[DRY-RUN] Would delete file\x1B[0m {:?}", path);
            } else if path.is_dir() {
                println!("  \x1B[90m[DRY-RUN] Would delete dir\x1B[0m {:?}", path);
            }
        } else {
            if path.is_file() {
                if let Ok(meta) = entry.metadata() {
                    freed_bytes += meta.len();
                }
                match fs::remove_file(&path) {
                    Ok(_) => deleted_count += 1,
                    Err(e) => println!("  \x1B[31m[FAILED]\x1B[0m {:?} - {}", path, e),
                }
            } else if path.is_dir() {
                let (sub_count, sub_bytes) = wipe_cache_dir_recursive(&path);
                deleted_count += sub_count;
                freed_bytes += sub_bytes;
                let _ = fs::remove_dir(&path); // Try remove empty dir
            }
        }
    }

    (deleted_count, freed_bytes)
}

fn wipe_cache_dir_recursive(dir: &Path) -> (usize, u64) {
    let mut count: usize = 0;
    let mut bytes: u64 = 0;

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return (0, 0),
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Ok(meta) = entry.metadata() {
                bytes += meta.len();
            }
            if fs::remove_file(&path).is_ok() {
                count += 1;
            }
        } else if path.is_dir() {
            let (sub_count, sub_bytes) = wipe_cache_dir_recursive(&path);
            count += sub_count;
            bytes += sub_bytes;
            let _ = fs::remove_dir(&path);
        }
    }

    (count, bytes)
}

/// Safety check: only delete from known cache locations
fn is_safe_to_delete(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    
    let safe_patterns = [
        "cache", "temp", "tmp", "thumbnail", "cookies", "history",
        "appData\\local", "appdata\\roaming", "application data",
        ".cache", "caches", "code cache", "gpuCache",
    ];

    let dangerous_patterns = [
        "windows\\system32", "program files", "syswow64",
        "boot", "drivers", "registry", ".sys", ".dll",
        "users\\<user>\\documents", "users\\<user>\\desktop",
        "users\\<user>\\downloads", "users\\<user>\\pictures",
    ];

    // Block dangerous paths
    for bad in &dangerous_patterns {
        if path_str.contains(bad) {
            return false;
        }
    }

    // Must contain at least one safe pattern
    safe_patterns.iter().any(|safe| path_str.contains(safe))
}

/// Prompt user for confirmation
pub fn confirm_wipe(message: &str) -> bool {
    print!("\n\x1B[33m{}\x1B[0m [y/N]: ", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim().to_lowercase();
    trimmed == "y" || trimmed == "yes"
}

/// Format bytes to human-readable
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}