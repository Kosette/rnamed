use clap::Parser;
use glob::glob;
use rayon::prelude::*;
use rnamed::rnamed;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Parser)]
#[command(version,about,long_about=None,arg_required_else_help(true))]
struct Args {
    /// Turn on recursively renaming files
    #[arg(short, long)]
    recursive: bool,
    /// Turn on glob patterns matching files and folders
    #[arg(short, long)]
    glob: bool,
    /// Paths provided to be processed
    paths: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let is_recursive = args.recursive;

    let globbing_enabled = args.glob;

    // Use a Mutex to safely share mutable data across threads
    let existing_files = Mutex::new(HashSet::new());

    args.paths[..].par_iter().for_each(|path| {
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
