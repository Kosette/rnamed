use blake3::Hasher;
use std::fs::{self, File};
use std::io::{self, Read};
use std::env;
use std::path::PathBuf;
use rayon::prelude::*;
use glob::glob;
use std::sync::Mutex;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Use a Mutex to safely share mutable data across threads
    let existing_files = Mutex::new(HashSet::new());

    // Check if globbing is enabled
    let globbing_enabled = args.contains(&"--glob".to_string()) || args.contains(&"-g".to_string());

    // Filter out the program name and the switch
    let paths = args.into_iter()
        .filter(|arg| arg != "--glob" && arg != "-g")
        .collect::<Vec<String>>();

    if paths.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No paths provided"));
    }

    paths.par_iter().for_each(|path| {
        if globbing_enabled {
            // If globbing is enabled, interpret the path as a glob pattern
            for entry in glob(path).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            check_and_rename(&path, &existing_files).expect("Failed to rename file");
                        }
                    }
                    Err(e) => eprintln!("Glob error: {:?}", e),
                }
            }
        } else {
            // No globbing, treat the path as a regular path
            let path = PathBuf::from(path);
            if path.is_file() {
                check_and_rename(&path, &existing_files).expect("Failed to rename file");
            } else if path.is_dir() {
                rename_files_in_directory(path, &existing_files).expect("Failed to rename files in directory");
            }
        }
    });

    let existing_files = existing_files.into_inner().unwrap();
    if !existing_files.is_empty() {
        println!("The following files were not renamed because a file with the checksum name already exists:");
        for file in existing_files {
            println!("{:?}", file);
        }
    }

    Ok(())
}

fn check_and_rename(file_path: &PathBuf, existing_files: &Mutex<HashSet<PathBuf>>) -> io::Result<()> {
    let mut file = File::open(&file_path)?;
    let mut hasher = Hasher::new();
    let mut buffer = Vec::new();

    // Read the file and feed it to the hasher
    file.read_to_end(&mut buffer)?;
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
        fs::rename(&file_path, &new_path)?;
        println!("Renamed {:?} to {:?}", file_path, new_path);
    }

    Ok(())
}

fn rename_files_in_directory(dir: PathBuf, existing_files: &Mutex<HashSet<PathBuf>>) -> io::Result<()> {
    let files = fs::read_dir(dir)?;

    files
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>()
        .par_iter()
        .for_each(|path| {
            check_and_rename(path, existing_files).expect("Failed to rename file");
        });

    Ok(())
}
