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

//! Command-line argument parsing for rustpix.
//!
//! This module handles all command-line argument parsing. It defines the argument
//! structure and provides functions to parse user input, validate options, and
//! display help/usage information.

// Program version, extracted from Cargo metadata.
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Args {
    /// List of input image files to process (supports wildcards when expanded by shell)
    pub files: Vec<String>,
    /// Optional base name for output files; if None, auto-generated names are used
    pub output: Option<String>,
    /// Whether to preserve original files after conversion (default: false)
    pub keep_original: bool,
    /// WebP compression quality level, range up to 100 (default: 75)
    pub quality: f32,
    /// Maximum target file size in bytes; if set, quality will be automatically adjusted
    pub max_size: Option<u64>,
    /// Whether to show compression statistics (default: false)
    pub show_stats: bool,
    /// Whether to process directories recursively
    pub recursive: bool,
    /// Dry-run mode: show what would be done without making changes
    pub dry_run: bool,
    /// Verbosity level: 0 = quiet, 1 = normal (default), 2 = verbose
    pub verbosity: u8,
    /// Use short UUID (8 chars) for output filenames instead of full UUID
    pub short_id: bool,
}

/// This function processes all command-line arguments, validates them, and returns
/// a structured representation. It handles flags, options with values, and file lists.
/// The function will exit the program if invalid arguments are provided or if
/// help/version information is requested.
pub fn parse_args() -> Args {
    // Collect command-line arguments, skipping the program name (first argument)
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // Initialize option variables with default values
    let mut output = None;
    let mut keep_original = false;
    let mut quality = 75.0; // Default WebP quality (good balance of size/quality)
    let mut max_size = None;
    let mut show_stats = false;
    let mut recursive = false;
    let mut dry_run = false;
    let mut verbosity: u8 = 1; // 0 = quiet, 1 = normal, 2 = verbose
    let mut short_id = false;

    // Show usage instructions if no arguments provided (user needs guidance)
    if args.is_empty() {
        print_usage_and_exit();
    }

    // Check for version flag and display version information if requested
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("Rustpix v{} (ALPHA)", VERSION);
        println!();
        println!("⚠️  WARNING: This software is in alpha stage.");
        println!("   Features may be incomplete, unstable, or change without notice.");
        println!();
        println!("Copyright (C) 2024-2026 Andre Franca");
        println!("Licensed under the GNU AGPL v3.0 or later.");
        println!("See <https://www.gnu.org/licenses/agpl-3.0.html> for details.");
        std::process::exit(0);
    }

    // Check for help flag and display detailed help information if requested
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help_and_exit();
    }

    // Parse output filename option (-o or --output)
    // This option requires a value (the desired output filename/pattern)
    if let Some(pos) = args.iter().position(|x| x == "-o" || x == "--output") {
        if pos + 1 < args.len() {
            output = Some(args.remove(pos + 1)); // Extract and store the output filename
            args.remove(pos); // Remove the flag itself from the argument list
        } else {
            eprintln!("Error: Missing output filename after -o or --output");
            std::process::exit(1);
        }
    }

    // Parse keep-original flag (-k or --keep-original)
    // This is a boolean flag that doesn't require a value
    if args.contains(&"-k".to_string()) || args.contains(&"--keep-original".to_string()) {
        keep_original = true;
        args.retain(|x| x != "-k" && x != "--keep-original"); // Remove flag from argument list
    }

    // Parse quality option (-q or --quality) with validation
    // Quality must be a floating-point number between 1.0 and 100.0
    if let Some(pos) = args.iter().position(|x| x == "-q" || x == "--quality") {
        if pos + 1 < args.len() {
            if let Ok(q) = args[pos + 1].parse::<f32>() {
                if (1.0..=100.0).contains(&q) {
                    quality = q; // Store validated quality value
                    args.remove(pos + 1); // Remove quality value from arguments
                    args.remove(pos); // Remove the flag from arguments
                } else {
                    eprintln!("Error: Quality must be a number between 1 and 100.");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: Invalid value for quality. It must be a number.");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: Missing value after -q or --quality.");
            std::process::exit(1);
        }
    }

    // Parse max-size option (-m or --max-size)
    // Max size must be specified in bytes, KB, or MB (e.g., "500KB", "2MB", "1000000")
    if let Some(pos) = args.iter().position(|x| x == "-m" || x == "--max-size") {
        if pos + 1 < args.len() {
            let size_str = &args[pos + 1];
            match parse_size(size_str) {
                Some(size) => {
                    max_size = Some(size);
                    args.remove(pos + 1);
                    args.remove(pos);
                }
                None => {
                    eprintln!("Error: Invalid max-size value. Use bytes, KB, or MB (e.g., '500KB', '2MB').");
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Error: Missing value after -m or --max-size.");
            std::process::exit(1);
        }
    }

    // Parse stats flag (-s or --stats)
    // This is a boolean flag that doesn't require a value
    if args.contains(&"-s".to_string()) || args.contains(&"--stats".to_string()) {
        show_stats = true;
        args.retain(|x| x != "-s" && x != "--stats");
    }

    // Parse recursive flag (-r or --recursive)
    // Process directories recursively
    if args.contains(&"-r".to_string()) || args.contains(&"--recursive".to_string()) {
        recursive = true;
        args.retain(|x| x != "-r" && x != "--recursive");
    }

    // Parse dry-run flag (-n or --dry-run)
    // Show what would be done without actually processing files
    if args.contains(&"-n".to_string()) || args.contains(&"--dry-run".to_string()) {
        dry_run = true;
        args.retain(|x| x != "-n" && x != "--dry-run");
    }

    // Parse verbose flag (-V or --verbose)
    // Increase output verbosity
    if args.contains(&"-V".to_string()) || args.contains(&"--verbose".to_string()) {
        verbosity = 2;
        args.retain(|x| x != "-V" && x != "--verbose");
    }

    // Parse quiet flag (-Q or --quiet)
    // Suppress output (overrides verbose)
    if args.contains(&"-Q".to_string()) || args.contains(&"--quiet".to_string()) {
        verbosity = 0;
        args.retain(|x| x != "-Q" && x != "--quiet");
    }

    // Parse short-id flag (--short-id)
    // Use short UUID (8 characters) instead of full UUID for output names
    if args.contains(&"--short-id".to_string()) {
        short_id = true;
        args.retain(|x| x != "--short-id");
    }

    // Return the parsed arguments as a structured Args object
    // Remaining args are treated as input file paths
    Args {
        files: args,
        output,
        keep_original,
        quality,
        max_size,
        show_stats,
        recursive,
        dry_run,
        verbosity,
        short_id,
    }
}

/// Parses a size string like "500KB", "2MB", or plain bytes into a u64 value
fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    if let Some(num) = s.strip_suffix("KB") {
        num.trim().parse::<u64>().ok().map(|n| n * 1024)
    } else if let Some(num) = s.strip_suffix("MB") {
        num.trim().parse::<u64>().ok().map(|n| n * 1024 * 1024)
    } else if let Some(num) = s.strip_suffix("GB") {
        num.trim().parse::<u64>().ok().map(|n| n * 1024 * 1024 * 1024)
    } else {
        s.parse::<u64>().ok()
    }
}

/// This function is called when no arguments are provided or when the user
/// needs basic usage guidance. It shows the command syntax and exits with code 1.
fn print_usage_and_exit() {
    eprintln!("Rustpix v{} (ALPHA) - Image optimization for the web", VERSION);
    eprintln!();
    eprintln!(
        "Usage: rustpix <file1> [file2 ...] [-o <output>] [-k] [-q <quality>] [-m <max-size>] [-s] [-r] [-n] [-V|-Q]"
    );
    eprintln!();
    eprintln!("Supported formats: PNG, JPEG, GIF, BMP, ICO, TIFF, SVG, HEIC, HEIF");
    eprintln!("Run with --help for more information.");
    std::process::exit(1);
}

/// This function provides detailed information about all available options,
/// supported file formats, and usage examples. It exits with code 0 since
/// help was explicitly requested by the user.
fn print_help_and_exit() {
    println!("Rustpix v{} (ALPHA)", VERSION);
    println!();
    println!("⚠️  WARNING: This software is in alpha stage.");
    println!("   Features may be incomplete, unstable, or change without notice.");
    println!();
    println!(
        "Usage: rustpix <file1> [file2 ...] [options]"
    );
    println!();
    println!("Supported formats:");
    println!("  Raster: PNG, JPEG, GIF (animated), BMP, ICO, TIFF");
    println!("  Vector: SVG");
    println!("  Apple:  HEIC, HEIF");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help message and exit");
    println!("  -v, --version        Show the version information and exit");
    println!("  -o, --output         Specify the output filename (without extension)");
    println!("  -k, --keep-original  Keep the original file after conversion");
    println!("  -q, --quality        Set the WebP quality (1-100). Default is 75.");
    println!("  -m, --max-size       Target maximum file size (e.g., '500KB', '2MB')");
    println!("  -s, --stats          Show compression statistics (before/after sizes)");
    println!("  -r, --recursive      Process directories recursively");
    println!("  -n, --dry-run        Show what would be done without processing");
    println!("  -V, --verbose        Increase output verbosity");
    println!("  -Q, --quiet          Suppress all output except errors");
    println!("      --short-id       Use short UUID (8 chars) for auto-generated names");
    println!();
    println!("Examples:");
    println!("  rustpix image.png                    Convert to WebP with default quality");
    println!("  rustpix *.jpg -q 85                  Convert all JPEGs with quality 85");
    println!("  rustpix photo.heic -m 500KB          Convert HEIC, target max 500KB");
    println!("  rustpix *.png -s -k                  Convert PNGs, show stats, keep originals");
    println!("  rustpix ./images -r                  Convert all images in directory tree");
    println!("  rustpix *.jpg -n                     Dry run: show what would be converted");
    println!("  rustpix img.png --short-id           Use short ID: abc12345.webp");
    println!();
    std::process::exit(0);
}
