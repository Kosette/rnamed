pub mod rnamed {
    use blake3::Hasher;
    use std::collections::HashSet;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::Mutex;

    pub fn check_and_rename(file_path: &PathBuf, existing_files: &Mutex<HashSet<PathBuf>>) {
        let mut file = fs::File::open(&file_path).unwrap();
        let mut hasher = Hasher::new();
        let mut buffer = Vec::new();

        // Read the file and feed it to the hasher
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
        let result = hasher.finalize();

        let checksum = format!("{}", result);
        let new_name = match file_path.extension() {
            Some(ext) => format!("{}.{}", checksum.to_uppercase(), ext.to_string_lossy()),
            None => checksum,
        };

        // Create a new path for the renamed file
        let new_path = file_path.with_file_name(new_name);

        if new_path.exists() {
            // If the target file name already exists, add it to the HashSet
            existing_files.lock().unwrap().insert(file_path.clone());
        } else {
            // Rename the file
            fs::rename(&file_path, &new_path).expect("rename files failed");
        }
    }

    pub fn rename_files_in_directory(dir: PathBuf, existing_files: &Mutex<HashSet<PathBuf>>) {
        // let paths = fs::read_dir(dir)?;
        match fs::read_dir(dir) {
            Ok(paths) => paths.into_iter().filter_map(|e| e.ok()).for_each(|e| {
                let e = e.path();
                if e.is_dir() {
                    rename_files_in_directory(e, &existing_files);
                } else {
                    check_and_rename(&e, &existing_files);
                }
            }),
            Err(_) => {
                println!("Error reading dir");
            }
        }
    }
}
