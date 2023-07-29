use glob::glob;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    for pattern in &args[1..] {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            let path = entry.unwrap();
            if path.is_dir() {
                process_dir(&path);
            } else if path.is_file() {
                rename_file(&path);
            }
        }
    }
}

fn process_dir(dir: &Path) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            process_dir(&path);
        } else {
            rename_file(&path);
        }
    }
}

fn rename_file(path: &Path) {
    let new_name = get_new_name(path);
    if Path::new(&new_name).exists() {
        println!("File {} already exists, skipping", new_name.display());
    } else {
        fs::rename(path, new_name).unwrap();
    }
}

fn get_new_name(path: &Path) -> PathBuf {
    let ext = path.extension().unwrap().to_str().unwrap();
    let name = format!("{}.{}", get_sha256(path).to_uppercase(), ext);
    path.with_file_name(name)
}

fn get_sha256(path: &Path) -> String {
    let data = fs::read(path).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}
