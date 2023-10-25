use blake3::Hasher;
use glob::glob;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    args[1..].par_iter().for_each(|pattern| {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            let path = entry.unwrap();
            if path.is_dir() {
                process_dir(&path);
            } else if path.is_file() {
                rename_file(&path);
            }
        }
    });
}

fn process_dir(dir: &Path) {
    match fs::read_dir(dir) {
        Ok(paths) => paths.into_iter().filter_map(|e| e.ok()).for_each(|e| {
            let path = e.path();
            if path.is_dir() {
                process_dir(&path);
            } else {
                rename_file(&path);
            }
        }),
        Err(err) => {
            eprintln!("Error reading dir {:?}: {}", dir, err);
        }
    }
}

fn rename_file(path: &Path) {
    let new_name = get_new_name(path);
    if new_name.exists() {
        println!("File {} already exists, skipping", new_name.display());
    } else {
        fs::rename(path, &new_name).unwrap();
    }
}

fn get_new_name(path: &Path) -> PathBuf {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let name = format!("{}.{}", get_blake3(path).to_uppercase(), ext);
    path.with_file_name(name)
}

fn get_blake3(path: &Path) -> String {
    let data = fs::read(path).unwrap();
    let mut hasher = Hasher::new();
    hasher.update(&data);
    let hash = hasher.finalize();
    format!("{}", hash)
}
