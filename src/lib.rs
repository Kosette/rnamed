pub mod rnamed {
    use blake3::Hasher;
    use glob::glob;
    use md5::{Digest, Md5};
    use sha2::{Sha256, Sha512};
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    pub fn search_files(
        path: impl AsRef<Path>,
        patterns: Option<Vec<String>>,
        options: &Options,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut results = Vec::new();

        if patterns.is_some() {
            let glob_patterns = if options.recursive {
                patterns
                    .unwrap()
                    .iter()
                    .map(|p| format!("{}/**/{}", path.as_ref().display(), p))
                    .collect::<Vec<String>>()
            } else {
                patterns
                    .unwrap()
                    .iter()
                    .map(|p| format!("{}/{}", path.as_ref().display(), p))
                    .collect::<Vec<String>>()
            };

            for p in glob_patterns.iter() {
                for entry in glob(p)? {
                    match entry {
                        Ok(path) => {
                            if path.is_file() {
                                results.push(path);
                            }
                        }
                        Err(e) => println!("Glob error: {:?}", e),
                    }
                }
            }
        } else {
            search_path(path.as_ref(), &mut results, options)?;
        }

        Ok(results)
    }

    fn search_path(
        path: &Path,
        results: &mut Vec<PathBuf>,
        options: &Options,
    ) -> Result<(), Box<dyn Error>> {
        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let path = entry?.path();

                if path.is_file() {
                    results.push(path);
                } else if options.recursive && path.is_dir() {
                    search_path(&path, results, options)?;
                }
            }
        } else if path.is_file() {
            results.push(path.to_path_buf());
        }

        Ok(())
    }

    pub enum Algo {
        Md5,
        Blake3,
        Sha256,
        Sha512,
    }

    pub struct Options {
        pub recursive: bool,
        pub algo: Algo,
        pub dry_run: bool,
    }

    pub fn check_and_rename(
        file_path: &PathBuf,
        options: &Options,
        existing_files: &Mutex<HashMap<PathBuf, PathBuf>>,
    ) {
        let checksum = match options.algo {
            Algo::Blake3 => b3_sum(file_path),
            Algo::Md5 => md5_sum(file_path),
            Algo::Sha256 => sha256_sum(file_path),
            Algo::Sha512 => sha512_sum(file_path),
        };

        let new_name = match file_path.extension() {
            Some(ext) => format!("{}.{}", checksum.to_uppercase(), ext.to_string_lossy()),
            None => checksum,
        };

        let new_path = file_path.with_file_name(new_name);

        if !options.dry_run {
            if new_path.exists() {
                existing_files
                    .lock()
                    .unwrap()
                    .insert(file_path.clone(), new_path);
            } else {
                fs::rename(file_path, &new_path).expect("rename files failed");
            }
        } else {
            existing_files
                .lock()
                .unwrap()
                .insert(file_path.clone(), new_path);
        }
    }

    fn md5_sum(file_path: &PathBuf) -> String {
        let file = fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::with_capacity(5_242_880, file);

        let mut hasher = Md5::new();

        let mut buffer = vec![0; 5_242_880];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => panic!("Error reading file: {}", e),
            }
        }

        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn b3_sum(file_path: &PathBuf) -> String {
        let file = fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::with_capacity(5_242_880, file);

        let mut hasher = Hasher::new();

        let mut buffer = vec![0; 5_242_880];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => panic!("Error reading file: {}", e),
            }
        }

        let result = hasher.finalize();

        format!("{}", result)
    }

    fn sha256_sum(file_path: &PathBuf) -> String {
        let file = fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::with_capacity(5_242_880, file);

        let mut hasher = Sha256::new();

        let mut buffer = vec![0; 5_242_880];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => panic!("Error reading file: {}", e),
            }
        }

        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn sha512_sum(file_path: &PathBuf) -> String {
        let file = fs::File::open(file_path).unwrap();
        let mut reader = std::io::BufReader::with_capacity(5_242_880, file);

        let mut hasher = Sha512::new();

        let mut buffer = vec![0; 5_242_880];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => panic!("Error reading file: {}", e),
            }
        }

        let result = hasher.finalize();

        format!("{:x}", result)
    }
}
