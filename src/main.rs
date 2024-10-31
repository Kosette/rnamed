use clap::Parser;
use console::{style, Emoji};
use indicatif::{HumanDuration, ParallelProgressIterator, ProgressStyle};
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
    /// Printing results only, no renaming
    #[arg(short, long)]
    dry_run: bool,
}

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨  ", ":-)");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static ROCKET: Emoji<'_, '_> = Emoji("🚀  ", "");

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let options = Options {
        recursive: args.recursive,
        algo: match args.algo.as_deref() {
            Some("md5") => Algo::Md5,
            Some("blake3") | Some("b3") => Algo::Blake3,
            Some("sha256") => Algo::Sha256,
            Some("sha512") => Algo::Sha512,
            None => Algo::Blake3,
            _ => {
                return Err("Unrecognized algorithm, use `--help` to see more.".into());
            }
        },
        dry_run: args.dry_run,
    };

    let start = std::time::Instant::now();

    println!(
        "{} {}Searching files...",
        style("[1/3]").bold().dim(),
        LOOKING_GLASS
    );

    let existing_files = Mutex::new(HashMap::new());
    let files_list = search_files(args.path, args.pattern, &options)?;

    let spinner_style = ProgressStyle::with_template("{spinner:.green} {msg}").unwrap();

    println!("{} {}Renaming files...", style("[2/3]").bold().dim(), CLIP);

    files_list[..]
        .par_iter()
        .progress_with_style(spinner_style)
        .with_message(format!("{}Working in progress, waiting...", ROCKET))
        .for_each(|path| {
            check_and_rename(path, &options, &existing_files);
        });

    println!(
        "{} {}Done in {}",
        style("[3/3]").bold().dim(),
        SPARKLE,
        HumanDuration(start.elapsed())
    );

    if !args.silent {
        let existing_files = existing_files.into_inner().unwrap();
        if !existing_files.is_empty() && !options.dry_run {
            println!("{}These files already exist:", PAPER);
            for file in existing_files {
                println!("{} => {}", file.0.display(), file.1.display());
            }
        } else if !existing_files.is_empty() {
            println!("{}These files will be renamed:", PAPER);
            for file in existing_files {
                println!("{} => {}", file.0.display(), file.1.display());
            }
        }
    }
    Ok(())
}
