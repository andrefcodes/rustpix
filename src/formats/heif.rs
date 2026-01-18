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

//! HEIC/HEIF decoding functionality.
//!
//! This module provides specialized decoding for Apple's HEIC and HEIF image formats.
//! It handles format detection through file extensions and magic number validation,
//! then uses libheif for actual decoding operations.

use image::{DynamicImage, ImageBuffer, Rgb};
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Determines if a file is in HEIC/HEIF format using a two-stage detection process:
/// 1. Primary check: Reads file header to verify format signature (most reliable)
/// 2. Fallback check: Examines file extension for common HEIC/HEIF extensions
///
/// The function looks for the 'ftyp' box signature at byte offset 4, which is
/// characteristic of ISO Base Media File Format (used by HEIC/HEIF).
/// Content validation is prioritized over file extensions to prevent misidentification
/// of files with incorrect extensions.
///
/// # Arguments
/// * `file_path` - Path to the image file to be analyzed
///
/// # Returns
/// * `true` - File is confirmed to be in HEIC/HEIF format
/// * `false` - File is not in HEIC/HEIF format or cannot be determined
pub fn is_heif_format(file_path: &Path) -> bool {
    // Check if file has the expected HEIC/HEIF extension
    let has_heif_extension = if let Some(ext) = file_path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let ext_lower = ext_str.to_lowercase();
            ext_lower == "heic" || ext_lower == "heif"
        } else {
            false
        }
    } else {
        false
    };

    // Check if file has valid HEIC/HEIF content signature
    let has_heif_content = if let Ok(mut file) = File::open(file_path) {
        let mut header = [0; 12]; // Read first 12 bytes for format detection
        if file.read_exact(&mut header).is_ok() {
            // HEIC/HEIF files follow ISO Base Media File Format with 'ftyp' box at offset 4
            if &header[4..8] == b"ftyp" {
                // Check for HEIF-compatible brand identifiers in the ftyp box
                let brand = &header[8..12];
                // 'heic' = HEIC images, 'heix' = HEIC image sequences
                // 'hevc' = HEVC codec, 'mif1' = Media Independent Format
                brand == b"heic" || brand == b"heix" || brand == b"hevc" || brand == b"mif1"
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    // Require BOTH valid content AND correct extension for positive identification
    // This prevents misidentification of files with wrong extensions or corrupted content
    has_heif_content && has_heif_extension
}

/// This function handles the complete HEIC/HEIF decoding pipeline:
/// 1. Reads the binary file data into memory
/// 2. Creates a libheif context for parsing the HEIF container
/// 3. Extracts the primary image from the container (HEIF can contain multiple images)
/// 4. Decodes the compressed image data to RGB format
/// 5. Converts the decoded data into Rust's standard image representation
///
/// # Arguments
/// * `path` - Filesystem path to the HEIC/HEIF image file
///
/// # Returns
/// * `Ok(DynamicImage)` - Successfully decoded image ready for further processing
/// * `Err(Box<dyn Error>)` - Decoding failed due to file I/O, format, or libheif errors
///
/// # System Requirements
/// * libheif must be installed on the system (libheif >= 1.17)
/// * Sufficient memory to load the entire image into RAM during decoding
///
/// # Errors
/// This function will return an error if:
/// * File cannot be read (permissions, missing file, etc.)
/// * File is corrupted or not a valid HEIC/HEIF format
/// * libheif fails to decode the image (unsupported codec, etc.)
/// * System runs out of memory during decoding
pub fn decode(path: &Path) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    // Step 1: Load the entire HEIC/HEIF file into memory for libheif processing
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Step 2: Initialize libheif context from the file data
    // HeifContext parses the HEIF container structure and metadata
    let ctx = HeifContext::read_from_bytes(&buffer)?;

    // Step 3: Get the primary image handle from the container
    // HEIF files can contain multiple images; we want the main/primary one
    let handle = ctx.primary_image_handle()?;

    // Step 4: Initialize LibHeif and decode the compressed image data to RGB format
    // Using default decoding options for maximum compatibility
    let lib_heif = LibHeif::new();
    let heif_image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    // Step 5: Extract image dimensions for buffer creation
    let width = heif_image.width();
    let height = heif_image.height();

    // Step 6: Extract the decoded RGB pixel data from libheif's internal format
    let planes = heif_image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane found")?;

    // Step 7: Create a standard RGB image buffer from the decoded pixel data
    let rgb_image =
        ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, interleaved_plane.data.to_vec())
            .ok_or("Failed to create RGB image from HEIC data")?;

    // Step 8: Convert to DynamicImage for compatibility with the rest of rustpix
    Ok(DynamicImage::from(rgb_image))
}
