pub mod rnamed {
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::path::{Path, PathBuf};

    pub fn process_dir(dir: &Path) {
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

    pub fn rename_file(path: &Path) {
        let new_name = get_new_name(path);
        if new_name.exists() {
            println!("File {} already exists, skipping", new_name.display());
        } else {
            fs::rename(path, &new_name).unwrap();
        }
    }

    fn get_new_name(path: &Path) -> PathBuf {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
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
}
