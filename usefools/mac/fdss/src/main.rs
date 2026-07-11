use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

const TARGET_FILE: &str = ".DS_Store";

fn main() -> notify::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        process::exit(1);
    }
    let watch_path = Path::new(&args[1]);
    if !watch_path.exists() || !watch_path.is_dir() {
        eprintln!("Usage: {} <directory_path>", args[0]);
        process::exit(1);
    }
    let abs_path = watch_path
        .canonicalize()
        .unwrap_or_else(|_| watch_path.to_path_buf());

    println!("Watching directory: {}", abs_path.display());
    println!("Target file: {}", TARGET_FILE);

    println!();
    println!("Cleaning existing files");
    delete_existing_files(&abs_path);
    println!("Finished cleaning existing files");
    println!();

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                    for path in event.paths {
                        check_and_delete(&path);
                    }
                }
            }
            Err(e) => eprintln!("Error occured: {:?}", e),
        }
    }
    Ok(())
}

fn delete_existing_files(path: &Path) {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    delete_existing_files(&entry_path);
                } else {
                    check_and_delete(&entry_path);
                }
            }
        }
    }
}

fn check_and_delete(path: &Path) {
    if let Some(file_name) = path.file_name() {
        if file_name == TARGET_FILE && path.exists() {
            let mut attempts = 0;
            while attempts < 3 {
                match fs::remove_file(path) {
                    Ok(_) => {
                        println!("Deleted: {}", path.display());
                        break;
                    }
                    Err(_) => {
                        attempts += 1;
                        thread::sleep(Duration::from_millis(50));
                    }
                }
            }
        }
    }
}
