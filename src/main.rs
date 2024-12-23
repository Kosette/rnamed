use eframe::egui;
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Default)]
struct RenamerApp {
    paths: Vec<PathBuf>,
    status: String,
    recursive: bool,
}

impl eframe::App for RenamerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Blake3 File Renamer");

            ui.add_space(10.0);

            // File selection buttons
            ui.horizontal(|ui| {
                if ui.button("Select Files").clicked() {
                    if let Some(files) = rfd::FileDialog::new().pick_files() {
                        self.paths = files;
                        self.status = format!("Selected {} files", self.paths.len());
                    }
                }
                if ui.button("Select Folder").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        self.paths = vec![folder];
                        self.status = "Selected 1 folder".to_string();
                    }
                }

                if ui.button("Clear Selections").clicked() {
                    self.clear_state();
                }
            });

            ui.add_space(10.0);
            // Recursive option
            ui.checkbox(&mut self.recursive, "Recursive folder search");

            ui.add_space(10.0);

            ui.label("Drag and drop files or folders here");

            let dropped_files = ui.input(|i| i.raw.dropped_files.clone());
            if !dropped_files.is_empty() {
                self.paths = dropped_files.into_iter().filter_map(|f| f.path).collect();
                self.status = format!("Dropped {} items", self.paths.len());
            }

            ui.add_space(10.0);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("Rename Files").clicked() {
                    self.rename_files();
                }
                ui.label(&self.status);
            });
        });
    }
}

impl RenamerApp {
    fn rename_files(&mut self) {
        let mut files_to_process = Vec::new();

        // Collect all files
        for path in &self.paths {
            if path.is_file() {
                files_to_process.push(path.clone());
            } else if path.is_dir() {
                let walker = if self.recursive {
                    WalkDir::new(path)
                } else {
                    WalkDir::new(path).max_depth(1)
                };

                files_to_process.extend(
                    walker
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|e| e.file_type().is_file())
                        .map(|e| e.path().to_path_buf()),
                );
            }
        }

        // Process files in parallel
        let results: Vec<_> = files_to_process
            .par_iter()
            .filter_map(|file| self.process_file(file))
            .collect();

        self.status = format!("Renamed {} files", results.len());
        self.paths.clear();
    }

    fn process_file(&self, file_path: &Path) -> Option<()> {
        let file = fs::File::open(file_path).ok()?;
        let mut reader = std::io::BufReader::with_capacity(5_242_880, file);

        let mut hasher = blake3::Hasher::new();

        let mut buffer = vec![0; 5_242_880];

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    hasher.update(&buffer[..n]);
                }
                Err(e) => panic!("Error reading file: {}", e),
            }
        }

        let hash = hasher.finalize();

        let hash_hex = hash.to_hex().to_uppercase();

        // Create new filename
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let new_name = if ext.is_empty() {
            hash_hex
        } else {
            format!("{}.{}", hash_hex, ext)
        };

        let new_path = file_path.with_file_name(new_name);

        // Skip if target file already exists
        if new_path.exists() {
            return None;
        }

        // Rename file
        fs::rename(file_path, new_path).ok()
    }

    fn clear_state(&mut self) {
        self.paths = Vec::new();
        self.status = String::new();
        self.recursive = false;
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "Blake3 File Renamer",
        options,
        Box::new(|_cc| Ok(Box::<RenamerApp>::default())),
    )
}
