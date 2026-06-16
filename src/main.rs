use std::{env, fs, path::Path};

use crate::resources::paths::CACHE_PATHS;

mod resources;

struct Display {
      folders:u32,
      files:u32,
      size:f64
}

fn main() {

    let mut stats = Display{
         folders:0,
         files:0,
         size:0.0
    };

    for path in &CACHE_PATHS {
        let resolved = resolve_path(path.path);
        let (files, folders, size) = scan_dir(&resolved);

        stats.size += size as f64  / 1024.0 / 1024.0;
        stats.files += files as u32;
        stats.folders += folders as u32;

        println!(
            "[PATH] {} [FILES] {} [FOLDERS] {} [SIZE] {} MB",
            path.name,
            files,
            folders,
            size as f64 / 1024.0 / 1024.0
        );

     
    }

       println!("\n \n Total cache data found \n Files : {} \n Folders : {} \n Size : {} MB " , stats.files , stats.folders , stats.size)
}

fn scan_dir(file_path: &str) -> (usize, usize, u64) {
    let path = Path::new(file_path);

    if !path.exists() {
        println!("  [SKIP] Does not exist: {}", file_path);
        return (0, 0, 0);
    }

    if !path.is_dir() {
        println!("  [SKIP] Not a directory: {}", file_path);
        return (0, 0, 0);
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            println!("  [DENIED] Cannot access {}: {}", file_path, e);
            return (0, 0, 0);
        }
    };

    let mut files = 0;
    let mut folders = 0;
    let mut total_size = 0u64;

    for entry in entries.filter_map(|e| e.ok()) {
        let p = entry.path();

        if p.is_dir() {
            folders += 1;
        } else if p.is_file() {
            files += 1;
            if let Ok(meta) = entry.metadata() {
                total_size += meta.len();
            }
        }
    }

    (files, folders, total_size)
}

fn resolve_path(path: &str) -> String {
    let user = env::var("USERNAME")
        .or_else(|_| env::var("USER"))
        .unwrap_or_default();

    path.replace("<user>", &user)
}