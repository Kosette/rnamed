use clap::Parser;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use rnamed::rnamed::{check_and_rename, search_files, Algo, Options};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

#[derive(Parser)]
#[command(version,about,long_about=None,arg_required_else_help(true))]
struct Args {
    /// Searching path or a single file path
    path: String,
    /// Set algorithm, default to `md5`. use `--help` to See more.
    ///
    /// "md5" => "Md5" | "b3" or "blake3" => "Blake3" | "sha256" => "Sha256" | "sha512" => "Sha512"
    #[arg(short, long)]
    algo: Option<String>,
    /// Enable globbing and provide patterns, can apply multi times, `-p "*.txt" -p "*.md"`
    #[arg(short, long)]
    pattern: Option<Vec<String>>,
    /// Turn on recursively searching
    #[arg(short, long)]
    recursive: bool,
    /// Turn on silent mode, suppressing existing files printing
    #[arg(short, long)]
    silent: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let options = Options {
        recursive: args.recursive,
        algo: match args.algo.as_deref() {
            Some("md5") => Algo::Md5,
            Some("blake3") | Some("b3") => Algo::Blake3,
            Some("sha256") => Algo::Sha256,
            Some("sha512") => Algo::Sha512,
            None => Algo::Md5,
            _ => {
                println!("Unrecognized algorithm, use `--help` to see more.");
                std::process::exit(-1);
            }
        },
    };

    let existing_files = Mutex::new(HashMap::new());

    let files_list = search_files(args.path, args.pattern, &options)?;

    files_list[..]
        .par_iter()
        .progress_count(files_list.len() as u64)
        .for_each(|path| {
            check_and_rename(path, &options.algo, &existing_files);
        });

    if !args.silent {
        let existing_files = existing_files.into_inner().unwrap();
        if !existing_files.is_empty() {
            println!("The following files already exist:");
            for file in existing_files {
                println!("{} => {}", file.0.display(), file.1.display());
            }
        }
    }
    Ok(())
}
