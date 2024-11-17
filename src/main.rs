// src/main.rs
use rayon::prelude::*;
use std::path::Path;
use rustpix::{args::parse_args, image_processing::process_image};

fn main() {
    // Collect command-line arguments (file paths or patterns)
    let args = parse_args();

    // Process each file in parallel
    args.par_iter().for_each(|file| {
        let path = Path::new(file);
        if let Err(e) = process_image(path) {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    });
}