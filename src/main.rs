use glob::glob;
use rayon::prelude::*;
use rnamed::rnamed;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No Path Found!");
        return;
    }

    if args.contains(&String::from("--help")) {
        println!("Usage: rnamed <path...>");
        println!("    Tip: multi paths and globs are accepted.");
        println!("    Warning: filename contains [] or other special characters may be ignored.");
        return;
    }

    args[1..].par_iter().for_each(|pattern| {
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            let path = entry.unwrap();
            if path.is_dir() {
                rnamed::process_dir(&path);
            } else if path.is_file() {
                rnamed::rename_file(&path);
            }
        }
    });
}
