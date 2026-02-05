// Copyright (C) 2024-2026 Andre Franca <andre@abf.li>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Rustpix - Image optimization for the web
//!
//! This is the main entry point for rustpix. It handles command-line argument parsing
//! and orchestrates parallel image processing operations across multiple input files.

mod cli;
mod formats;
mod processing;

use indicatif::{ProgressBar, ProgressStyle};
use processing::{ProcessingOptions, ProcessingResult};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use walkdir::WalkDir;

/// Supported image extensions for input
const SUPPORTED_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "tif",
    "heic",
    "heif",
    "svg",
    "webp",
    "avif",
];

/// Check if a path has a supported image extension
fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Collect files to process, optionally recursing into directories
fn collect_files(paths: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if path.is_dir() {
            if recursive {
                for entry in WalkDir::new(path)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() && is_supported_image(entry.path()) {
                        files.push(entry.path().to_path_buf());
                    }
                }
            } else {
                // Non-recursive: just list files in the directory
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let entry_path = entry.path();
                        if entry_path.is_file() && is_supported_image(&entry_path) {
                            files.push(entry_path);
                        }
                    }
                }
            }
        } else if path.is_file() && is_supported_image(path) {
            files.push(path.to_path_buf());
        } else if path.is_file() {
            eprintln!("⚠️  Skipping unsupported format: {}", path.display());
        }
    }

    files
}

/// Workflow:
/// 1. Parse command-line arguments to get input files, output options, and processing parameters
/// 2. Use parallel processing (via rayon) to handle multiple images concurrently
/// 3. For each file, determine the appropriate output filename based on user preferences
/// 4. Delegate actual image processing to the processing module
/// 5. Handle and report any errors that occur during processing
fn main() {
    // Parse command-line arguments into an Args struct containing all user preferences
    let args = cli::parse_args();

    // Collect files to process (handles directories and recursion)
    let files = collect_files(&args.files, args.recursive);

    if files.is_empty() {
        eprintln!("No supported image files found.");
        std::process::exit(1);
    }

    let file_count = files.len();
    let show_stats = args.show_stats;
    let verbosity = args.verbosity;
    let dry_run = args.dry_run;
    let format = args.format;

    // Dry-run mode: just show what would be done
    if dry_run {
        println!("Dry run: Would process {} files:", file_count);
        for file in &files {
            println!("  - {}", file.display());
        }
        if args.recursive {
            println!("  (recursive mode enabled)");
        }
        println!("  Output format: {}", format.extension());
        println!("  Quality: {}", args.quality);
        if let Some(max_size) = args.max_size {
            println!("  Max size: {} bytes", max_size);
        }
        if let Some((w, h)) = args.crop {
            println!("  Crop: {}x{} pixels", w, h);
        }
        println!("  Keep original: {}", args.keep_original);
        return;
    }

    // Create progress bar for batch operations (unless quiet mode)
    let progress_bar = if file_count > 1 && verbosity > 0 {
        let pb = ProgressBar::new(file_count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    // Collect results for statistics
    let results: Mutex<Vec<ProcessingResult>> = Mutex::new(Vec::new());
    let total_original: AtomicU64 = AtomicU64::new(0);
    let total_output: AtomicU64 = AtomicU64::new(0);
    let error_count: AtomicU64 = AtomicU64::new(0);

    // Process each input file in parallel using rayon's parallel iterator for improved performance
    // This allows multiple images to be processed simultaneously on multi-core systems
    files.par_iter().enumerate().for_each(|(index, file)| {
        let path = file.as_path(); // Use PathBuf from collected files

        // Determine output filename based on user preferences and number of input files
        let output = if let Some(ref output) = args.output {
            if files.len() == 1 {
                // Single input file: use the exact output name specified by user
                Some(output.clone())
            } else {
                // Multiple input files: append numerical suffix to avoid filename conflicts
                Some(format!("{}{}", output, index + 1))
            }
        } else {
            // No custom output specified: processing module will generate default names
            None
        };

        let options = ProcessingOptions {
            output,
            keep_original: args.keep_original,
            quality: args.quality,
            max_size: args.max_size,
            show_stats: args.show_stats,
            short_id: args.short_id,
            format,
            crop: args.crop,
        };

        // Delegate image processing to the processing module, handling any errors that occur
        match processing::process_image(path, options) {
            Ok(result) => {
                total_original.fetch_add(result.original_size, Ordering::Relaxed);
                total_output.fetch_add(result.output_size, Ordering::Relaxed);

                // Only print individual results if not in quiet mode
                if verbosity > 0 {
                    if show_stats || verbosity >= 2 {
                        println!(
                            "✓ {} → {} ({} → {}, {:.1}% reduction, q={})",
                            path.display(),
                            result.output_path.display(),
                            ProcessingResult::format_size(result.original_size),
                            ProcessingResult::format_size(result.output_size),
                            result.compression_ratio(),
                            result.quality_used as u32
                        );
                    } else {
                        println!("✓ Processed: {}", result.output_path.display());
                    }
                }

                results.lock().unwrap().push(result);
            }
            Err(e) => {
                eprintln!("✗ Error processing {:?}: {}", path, e);
                error_count.fetch_add(1, Ordering::Relaxed);
            }
        }

        // Update progress bar
        if let Some(ref pb) = progress_bar {
            pb.inc(1);
        }
    });

    // Finish progress bar
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Done");
    }

    // Print summary statistics for batch operations (unless quiet mode)
    let results = results.into_inner().unwrap();
    if results.len() > 1 && (show_stats || verbosity >= 2) && verbosity > 0 {
        let total_orig = total_original.load(Ordering::Relaxed);
        let total_out = total_output.load(Ordering::Relaxed);
        let errors = error_count.load(Ordering::Relaxed);

        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Summary: {} files processed", results.len());
        if errors > 0 {
            println!("         {} errors", errors);
        }
        println!(
            "         {} → {} (total)",
            ProcessingResult::format_size(total_orig),
            ProcessingResult::format_size(total_out)
        );
        if total_orig > 0 {
            let ratio = (1.0 - (total_out as f64 / total_orig as f64)) * 100.0;
            println!("         {:.1}% overall reduction", ratio);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}
