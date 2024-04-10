use glob::glob;
use rayon::prelude::*;
use rnamed::rnamed;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let print_msg = "Usage: rnamed [-r|--recursive] [-g|--glob] <Path...>\nTip: multiple paths and globs accepted.\nWarning: filename containing special characters like [] may be ignored.\n*Recusive operation may be memory intensive.*".to_string();

    // print help msg
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        println!("{}", print_msg);
        return;
    }

    // check if is_recursive
    let is_recursive =
        args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string());

    // Check if globbing is enabled
    let globbing_enabled = args.contains(&"--glob".to_string()) || args.contains(&"-g".to_string());

    // Filter out the program name and the switch
    let paths = args
        .into_iter()
        .filter(|arg| {
            arg != "--glob"
                && arg != "-g"
                && arg != "-r"
                && arg != "--recursive"
                && arg != "-h"
                && arg != "--help"
        })
        .collect::<Vec<String>>();

    if paths[1..].is_empty() {
        println!("Error: no path provided\n\n{}", print_msg);
    }

    // Use a Mutex to safely share mutable data across threads
    let existing_files = Mutex::new(HashSet::new());

    paths[1..].par_iter().for_each(|path| {
        if globbing_enabled {
            // If globbing is enabled, interpret the path as a glob pattern
            for entry in glob(path).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if path.is_file() {
                            rnamed::check_and_rename(&path, &existing_files);
                        } else if path.is_dir() && is_recursive {
                            rnamed::rename_files_in_directory(path, &existing_files, true);
                        } else {
                            rnamed::rename_files_in_directory(path, &existing_files, false);
                        }
                    }
                    Err(e) => eprintln!("Glob error: {:?}", e),
                }
            }
        } else {
            // No globbing, treat the path as a regular path
            let path = PathBuf::from(path);
            if path.is_file() {
                rnamed::check_and_rename(&path, &existing_files);
            } else if path.is_dir() && is_recursive {
                rnamed::rename_files_in_directory(path, &existing_files, true);
            } else {
                rnamed::rename_files_in_directory(path, &existing_files, false);
            }
        }
    });

    let existing_files = existing_files.into_inner().unwrap();
    if !existing_files.is_empty() {
        println!("The following files with their checksum name already exist:");
        for file in existing_files {
            println!("{:?}", file);
        }
    }
}
