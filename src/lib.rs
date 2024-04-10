pub mod rnamed {
    use blake3::Hasher;
    use rayon::prelude::*;
    use std::collections::HashSet;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;
    use std::sync::Mutex;

    pub fn check_and_rename(file_path: &PathBuf, existing_files: &Mutex<HashSet<PathBuf>>) {
        let mut file = fs::File::open(file_path).unwrap();
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
            fs::rename(file_path, &new_path).expect("rename files failed");
        }
    }

    pub fn rename_files_in_directory(
        dir: PathBuf,
        existing_files: &Mutex<HashSet<PathBuf>>,
        r: bool,
    ) {
        // let paths = fs::read_dir(dir)?;
        match fs::read_dir(dir) {
            Ok(paths) => {
                let mut file_vec = Vec::new();
                let mut dir_vec = Vec::new();

                paths
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .for_each(|e| {
                        if e.is_dir() {
                            dir_vec.push(e);
                        } else {
                            file_vec.push(e);
                        }
                    });
                file_vec
                    .par_iter()
                    .for_each(|e| check_and_rename(e, existing_files));
                if r {
                    dir_vec.par_iter().for_each(|e| {
                        rename_files_in_directory(e.to_path_buf(), existing_files, r)
                    });
                }
            }
            Err(_) => {
                println!("Error reading dir");
            }
        }
    }
}
