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

//! Core image processing functionality.
//!
//! This module handles decoding various image formats, converting them to WebP format
//! with configurable quality settings, and managing file operations.

use image::ImageReader;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

use crate::formats::heif;
use crate::formats::webp;
use crate::formats::svg;

/// Result of image processing, containing statistics about the operation
#[derive(Debug)]
pub struct ProcessingResult {
    /// Path to the output file
    pub output_path: std::path::PathBuf,
    /// Original file size in bytes
    pub original_size: u64,
    /// Output file size in bytes
    pub output_size: u64,
    /// Quality level used for encoding
    pub quality_used: f32,
    /// Whether the image was animated (for future animated WebP support)
    #[allow(dead_code)]
    pub is_animated: bool,
}

impl ProcessingResult {
    /// Calculate compression ratio as a percentage
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        (1.0 - (self.output_size as f64 / self.original_size as f64)) * 100.0
    }

    /// Format file size for human-readable display
    pub fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
}

/// Configuration for processing options
pub struct ProcessingOptions {
    pub output: Option<String>,
    pub keep_original: bool,
    pub quality: f32,
    pub max_size: Option<u64>,
    /// Show stats flag - currently handled by main.rs but kept here for future use
    #[allow(dead_code)]
    pub show_stats: bool,
    /// Use short UUID (8 characters) instead of full UUID for auto-generated names
    pub short_id: bool,
}

/// Generate a unique identifier for output filename
/// Uses UUID v7 (timestamp-based, sortable) for better chronological ordering
fn generate_id(short: bool) -> String {
    let uuid = Uuid::now_v7();
    if short {
        // Use first 8 characters of UUID for a shorter, more user-friendly name
        uuid.to_string()[..8].to_string()
    } else {
        uuid.to_string()
    }
}

/// Check if a file is an animated GIF
fn is_animated_gif(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension()
        && ext.to_str().map(|s| s.to_lowercase()) == Some("gif".to_string())
    {
        // Try to detect if GIF has multiple frames
        if let Ok(file) = File::open(file_path) {
            let decoder = image::codecs::gif::GifDecoder::new(std::io::BufReader::new(file));
            if let Ok(decoder) = decoder {
                use image::AnimationDecoder;
                // If we can get frames and there's more than one, it's animated
                return decoder.into_frames().count() > 1;
            }
        }
    }
    false
}

/// Check if a file is an SVG
fn is_svg_file(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension() {
        let ext_lower = ext.to_str().map(|s| s.to_lowercase());
        return ext_lower == Some("svg".to_string()) || ext_lower == Some("svgz".to_string());
    }
    false
}

/// This function handles the complete image processing pipeline:
/// 1. Detects if the input is a special format (HEIC/HEIF, SVG, animated GIF)
/// 2. Decodes the image using appropriate decoder
/// 3. Encodes the decoded image to WebP format with specified quality
/// 4. If max_size is specified, adjusts quality to meet size target
/// 5. Generates an appropriate output filename (custom or UUID-based)
/// 6. Saves the WebP file to disk
/// 7. Optionally removes the original file based on user preference
///
/// # Arguments
/// * `file_path` - Path to the input image file to be processed
/// * `options` - Processing options including output, quality, max_size, etc.
///
/// # Returns
/// * `Ok(ProcessingResult)` - Processing completed successfully with statistics
/// * `Err(Box<dyn std::error::Error>)` - Processing failed with detailed error information
pub fn process_image(
    file_path: &Path,
    options: ProcessingOptions,
) -> Result<ProcessingResult, Box<dyn std::error::Error + Send + Sync>> {
    // Get original file size for statistics
    let original_size = fs::metadata(file_path)?.len();

    // Check for animated GIF - these need special handling
    let is_animated = is_animated_gif(file_path);
    if is_animated {
        return process_animated_gif(file_path, options, original_size);
    }

    // Check for SVG - needs rasterization first
    if is_svg_file(file_path) {
        return process_svg(file_path, options, original_size);
    }

    // Detect image format to determine appropriate decoding strategy
    // HEIC/HEIF files require specialized handling due to their unique format
    let is_heif_format = heif::is_heif_format(file_path);

    // Decode the image using the appropriate decoder based on format detection
    let img = if is_heif_format {
        // Use specialized HEIF decoder for Apple's HEIC/HEIF formats
        heif::decode(file_path)?
    } else {
        // Use content-based format detection for standard formats (PNG, JPEG, GIF, BMP, etc.)
        // This handles files with incorrect extensions by analyzing file content instead of extension
        ImageReader::open(file_path)?
            .with_guessed_format()?
            .decode()?
    };

    // Convert and encode with quality adjustment if max_size is specified
    let (webp_data, quality_used) = if let Some(max_size) = options.max_size {
        encode_with_size_limit(&img, options.quality, max_size)?
    } else {
        let data = webp::encode(img, options.quality)?;
        (data, options.quality)
    };

    // Determine output filename based on user preference
    let new_file_name = if let Some(ref output) = options.output {
        // Use custom output name provided by user
        format!("{}.webp", output)
    } else {
        // Generate unique filename using UUID v7 (timestamp-based, sortable)
        format!("{}.webp", generate_id(options.short_id))
    };

    // Create full path for the new WebP file in the same directory as the original
    let new_file_path = file_path.with_file_name(&new_file_name);

    // Write the encoded WebP data to the output file
    let mut output_file = File::create(&new_file_path)?;
    output_file.write_all(&webp_data)?;

    let output_size = webp_data.len() as u64;

    // Remove original file if user hasn't requested to keep it
    if !options.keep_original {
        fs::remove_file(file_path)?;
    }

    Ok(ProcessingResult {
        output_path: new_file_path,
        original_size,
        output_size,
        quality_used,
        is_animated: false,
    })
}

/// Encode image with automatic quality adjustment to meet size target
fn encode_with_size_limit(
    img: &image::DynamicImage,
    initial_quality: f32,
    max_size: u64,
) -> Result<(Vec<u8>, f32), Box<dyn std::error::Error + Send + Sync>> {
    let mut quality = initial_quality;
    let mut webp_data = webp::encode(img.clone(), quality)?;

    // Binary search for optimal quality
    let mut min_quality = 1.0_f32;
    let mut max_quality = quality;

    // If initial quality already meets size requirement, try higher quality
    if (webp_data.len() as u64) <= max_size {
        // Already under target, but could we do better quality?
        max_quality = 100.0;
        while max_quality - quality > 1.0 {
            let try_quality = (quality + max_quality) / 2.0;
            let try_data = webp::encode(img.clone(), try_quality)?;
            if (try_data.len() as u64) <= max_size {
                quality = try_quality;
                webp_data = try_data;
            } else {
                max_quality = try_quality;
            }
        }
    } else {
        // Over target, need to reduce quality
        while max_quality - min_quality > 1.0 {
            quality = (min_quality + max_quality) / 2.0;
            webp_data = webp::encode(img.clone(), quality)?;
            if (webp_data.len() as u64) <= max_size {
                min_quality = quality;
            } else {
                max_quality = quality;
            }
        }
        // Final encode at the best quality that fits
        quality = min_quality;
        webp_data = webp::encode(img.clone(), quality)?;
    }

    Ok((webp_data, quality))
}

/// Process animated GIF files - convert to animated WebP
fn process_animated_gif(
    file_path: &Path,
    options: ProcessingOptions,
    original_size: u64,
) -> Result<ProcessingResult, Box<dyn std::error::Error + Send + Sync>> {
    use image::codecs::gif::GifDecoder;
    use image::AnimationDecoder;

    let file = File::open(file_path)?;
    let decoder = GifDecoder::new(std::io::BufReader::new(file))?;
    let frames: Vec<_> = decoder.into_frames().collect::<Result<Vec<_>, _>>()?;

    if frames.is_empty() {
        return Err("GIF has no frames".into());
    }

    // For animated GIFs, we'll encode each frame and create an animated WebP
    // Note: The webp crate doesn't support animated WebP encoding directly
    // So we'll convert to a static WebP using the first frame for now
    // and mark it as animated in the result
    let first_frame = &frames[0];
    let img = image::DynamicImage::ImageRgba8(first_frame.buffer().clone());

    let (webp_data, quality_used) = if let Some(max_size) = options.max_size {
        encode_with_size_limit(&img, options.quality, max_size)?
    } else {
        let data = webp::encode(img, options.quality)?;
        (data, options.quality)
    };

    let new_file_name = if let Some(ref output) = options.output {
        format!("{}.webp", output)
    } else {
        format!("{}.webp", generate_id(options.short_id))
    };

    let new_file_path = file_path.with_file_name(&new_file_name);

    let mut output_file = File::create(&new_file_path)?;
    output_file.write_all(&webp_data)?;

    let output_size = webp_data.len() as u64;

    if !options.keep_original {
        fs::remove_file(file_path)?;
    }

    // Note: Currently converts animated GIF to static WebP (first frame)
    // Full animated WebP support would require webp-animation crate
    eprintln!(
        "⚠️  Note: Animated GIF converted to static WebP (first frame only). \
         Full animation support coming soon."
    );

    Ok(ProcessingResult {
        output_path: new_file_path,
        original_size,
        output_size,
        quality_used,
        is_animated: true,
    })
}

/// Process SVG files - rasterize and convert to WebP
fn process_svg(
    file_path: &Path,
    options: ProcessingOptions,
    original_size: u64,
) -> Result<ProcessingResult, Box<dyn std::error::Error + Send + Sync>> {
    let img = svg::rasterize(file_path)?;

    let (webp_data, quality_used) = if let Some(max_size) = options.max_size {
        encode_with_size_limit(&img, options.quality, max_size)?
    } else {
        let data = webp::encode(img, options.quality)?;
        (data, options.quality)
    };

    let new_file_name = if let Some(ref output) = options.output {
        format!("{}.webp", output)
    } else {
        format!("{}.webp", generate_id(options.short_id))
    };

    let new_file_path = file_path.with_file_name(&new_file_name);

    let mut output_file = File::create(&new_file_path)?;
    output_file.write_all(&webp_data)?;

    let output_size = webp_data.len() as u64;

    if !options.keep_original {
        fs::remove_file(file_path)?;
    }

    Ok(ProcessingResult {
        output_path: new_file_path,
        original_size,
        output_size,
        quality_used,
        is_animated: false,
    })
}
