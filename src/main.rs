/**************************************************************************
 *                                                                        *
 *                              RustPix                                   *
 *        A Command-Line Tool for Image Optimization for the Web          *
 *                                                                        *
 *  Author: Andre Franca                                                  *
 *  Repository: https://github.com/a-franca/rustpix                       *
 *                                                                        *
 *  Licensing:                                                            *
 *  This project is dual-licensed under either of the following licenses: *
 *                                                                        *
 *  - Apache License (Version 2.0)                                        *
 *        (https://github.com/a-franca/rustpix/blob/main/LICENSE-APACHE)  *
 *  - MIT License                                                         *
 *        (https://github.com/a-franca/rustpix/blob/main/LICENSE-MIT)     *
 *                                                                        *
 *  You may choose either license to use this software.                   *
 *                                                                        *
 *  Description:                                                          *
 *                                                                        *
 *  RustPix is a fast and efficient tool for optimizing image files       *
 *  for the web. It supports batch processing, adjustable quality,        *
 *  and the option to retain original files. Built for performance        *
 *  and ease of use.                                                      *
 *                                                                        *
 **************************************************************************/

use rayon::prelude::*;
use image::{io::Reader as ImageReader, DynamicImage};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use webp::Encoder;

// Program version, extracted from Cargo metadata.
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Struct to hold parsed command-line arguments.
struct Args {
    files: Vec<String>,       // List of input files to process
    output: Option<String>,   // Base name for the output file(s)
    keep_original: bool,      // Flag to keep original files after conversion
    quality: f32,             // WebP quality level (1-100)
}

// Parses command-line arguments into the Args struct.
fn parse_args() -> Args {
    let mut args: Vec<String> = std::env::args().skip(1).collect(); // Skip program name
    let mut output = None;
    let mut keep_original = false;
    let mut quality = 75.0; // Default quality value

    // Display usage instructions if no arguments are provided.
    if args.is_empty() {
        eprintln!("Usage: rustpix <file1> [file2 ...] or rustpix *.<pattern> or rustpix * [-o <output>] [-k] [-q <quality>]");
        std::process::exit(1);
    }

    // Handle version flag (-v or --version)
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("rustpix version {}", VERSION);
        std::process::exit(0);
    }

    // Handle help flag (-h or --help)
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help_and_exit();
    }

    // Handle output flag (-o or --output)
    if let Some(pos) = args.iter().position(|x| x == "-o" || x == "--output") {
        if pos + 1 < args.len() {
            output = Some(args.remove(pos + 1)); // Capture the output filename
            args.remove(pos);                   // Remove the flag from arguments
        } else {
            eprintln!("Error: Missing output filename after -o or --output");
            std::process::exit(1);
        }
    }

    // Handle keep-original flag (-k or --keep-original)
    if args.contains(&"-k".to_string()) || args.contains(&"--keep-original".to_string()) {
        keep_original = true;
        args.retain(|x| x != "-k" && x != "--keep-original"); // Remove the flag from arguments
    }

    // Handle quality flag (-q or --quality)
    if let Some(pos) = args.iter().position(|x| x == "-q" || x == "--quality") {
        if pos + 1 < args.len() {
            if let Ok(q) = args[pos + 1].parse::<f32>() {
                if (1.0..=100.0).contains(&q) {
                    quality = q; // Update quality value
                    args.remove(pos + 1); // Remove quality value from arguments
                    args.remove(pos);     // Remove the flag from arguments
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

    // Return parsed arguments as an Args struct.
    Args {
        files: args,
        output,
        keep_original,
        quality,
    }
}

// Displays help message and exits.
fn print_help_and_exit() {
    println!();
    println!("Usage: rustpix <file1> [file2 ...] or rustpix *.<pattern> or rustpix * [-o <output>] [-k] [-q <quality>]");
    println!();
    println!("Options:");
    println!("  -h, --help           Show this help message and exit");
    println!("  -v, --version        Show the version information and exit");
    println!("  -o, --output         Specify the output filename");
    println!("  -k, --keep-original  Keep the original file after conversion");
    println!("  -q, --quality        Set the WebP quality (1-100). Default is 75.");
    println!();
    std::process::exit(0);
}

// Processes a single image file.
// Arguments:
// - file_path: Path to the input file.
// - output: Optional custom output filename.
// - keep_original: Whether to keep the original file after conversion.
// - quality: WebP quality level for the conversion.
fn process_image(file_path: &Path, output: Option<String>, keep_original: bool, quality: f32) -> Result<(), Box<dyn std::error::Error>> {
    
    // Load the image and decode it (stripping metadata like EXIF).
    let img = ImageReader::open(file_path)?.decode()?;
    
    // Encode the image to WebP format with the specified quality.
    let webp_data = encode_webp(img, quality)?;

    // Generate a new filename for the WebP file.
    let new_file_name = if let Some(output) = output {
        format!("{}.webp", output)
    } else {
        let uuid = Uuid::new_v4();
        format!("{}.webp", uuid)
    };
    let new_file_path = file_path.with_file_name(new_file_name);

    // Save the WebP file to disk.
    let mut output_file = File::create(&new_file_path)?;
    output_file.write_all(&webp_data)?;

    // Delete the original file unless keep_original is set.
    if !keep_original {
        fs::remove_file(file_path)?;
    }

    println!("Processed: {:?} with quality {}", new_file_path, quality);
    Ok(())
}

// Encodes an image into WebP format.
// Arguments:
// - img: The input image in `DynamicImage` format.
// - quality: The desired WebP quality level (1-100).
// Returns: A vector of bytes containing the encoded WebP data.
fn encode_webp(img: DynamicImage, quality: f32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let rgb = img.to_rgb8(); // Convert the image to RGB format
    let (width, height) = rgb.dimensions(); // Extract dimensions
    let encoder = Encoder::from_rgb(&rgb, width, height); // Create WebP encoder
    let webp = encoder.encode(quality); // Encode the image with the specified quality
    Ok(webp.to_vec()) // Return the encoded data as a vector of bytes
}

fn main() {
    // Parse command-line arguments into an Args struct.
    let args = parse_args();

    // Process each input file in parallel using rayon for improved performance.
    args.files.par_iter().enumerate().for_each(|(index, file)| {
        let path = Path::new(file); // Convert file name to Path
        let output = if let Some(ref output) = args.output {
            if args.files.len() == 1 {
                Some(output.clone()) // Use the specified output name for single input
            } else {
                Some(format!("{}{}", output, index + 1)) // Append index for multiple inputs
            }
        } else {
            None // No custom output specified
        };

        // Attempt to process the image, logging errors if any occur.
        if let Err(e) = process_image(path, output, args.keep_original, args.quality) {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    });
}