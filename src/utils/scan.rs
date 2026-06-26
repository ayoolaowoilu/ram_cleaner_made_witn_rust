use std::{env, fs, path::Path};


fn is_cache_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.ends_with(".cache")
        || lower.ends_with(".tmp")
        || lower.ends_with(".temp")
        || lower.ends_with(".pyc")
        || lower.ends_with(".pyo")
        || lower.ends_with(".log")
        || lower.ends_with(".old")
        || lower.ends_with(".bak")
        || lower.contains("cache")
        || lower.contains("thumbs.db")
        || name == ".DS_Store"
        || name.starts_with('.') && lower.contains("cache")
}

pub fn scan_dir(path: &str, pathname: &str) -> (u32, u32, f32) {
    let dir = Path::new(&path);

    if !dir.exists() {
        return (0, 0, 0.0);
    }

    if !dir.is_dir() {
        return (0, 0, 0.0);
    }

    let mut files: u32 = 0;
    let mut folders: u32 = 0;
    let mut size: f64 = 0.0;
    let mut cache_files: u32 = 0;
    let mut cache_size: f64 = 0.0;

    let count = match fs::read_dir(dir) {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("\n \x1B[31m[ERROR]:\x1B[0m Cannot Read {} , {} ", path, e);
            return (0, 0, 0.0);
        }
    };

    for entry in count.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if path.is_dir() {
            folders += 1;
        } else if path.is_file() {
            files += 1;

            if is_cache_file(name) {
                cache_files += 1;
                if let Ok(meta) = entry.metadata() {
                    cache_size += meta.len() as f64;
                }
            }
        }

        if let Ok(meta) = entry.metadata() {
            size += meta.len() as f64;
        }
    }

    let size_gb = ((size / 1024.0 / 1024.0 / 1024.0) * 100.0).round() / 100.0;
    let cache_gb = ((cache_size / 1024.0 / 1024.0 / 1024.0) * 100.0).round() / 100.0;

    println!("\n {pathname}");
    println!(" Files : {files}");
    println!(" Folders : {folders}");
    println!(" Size : {}GB", size_gb);
    println!(" Cache Files : {cache_files}");
    println!(" Cache Size : {}GB", cache_gb);

    (files, folders, size.floor() as f32)
}

pub fn resolve_paths(path: &str) -> String {
    let user: String = env::var("USERNAME")
        .or_else(|_| env::var("USER"))
        .unwrap_or_default();

    path.replace("<user>", &user)
}

/// Get user-accessible cache directories (no system paths that need admin)
pub fn get_user_cache_dirs() -> Vec<String> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = env::var("LOCALAPPDATA") {
            dirs.push(local.clone());
            dirs.push(format!("{}\\Temp", local));
            dirs.push(format!("{}\\Microsoft\\Windows\\Explorer", local)); // Thumbnails
        }
        if let Ok(roaming) = env::var("APPDATA") {
            dirs.push(roaming);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = env::var("HOME") {
            dirs.push(format!("{}/Library/Caches", home));
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = env::var("HOME") {
            dirs.push(format!("{}/.cache", home));
            dirs.push(format!("{}/.tmp", home));
        }
    }

    dirs
}

