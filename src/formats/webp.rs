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

//! WebP encoding functionality.
//!
//! This module handles the conversion of decoded images into optimized WebP format
//! with configurable quality.

use image::DynamicImage;
use std::error::Error;
use webp::Encoder;

/// # Arguments
/// * `img` - The decoded image as a `DynamicImage` (supports various pixel formats)
/// * `quality` - WebP compression quality level (where 100 = highest quality)
///
/// # Returns
/// * `Ok(Vec<u8>)` - Successfully encoded WebP image data as bytes
/// * `Err(Box<dyn Error>)` - Encoding failed due to invalid input or system constraints
///
/// # Examples
/// Encode with high quality (good for photos)
/// let webp_data = encode(image, 85.0)?;
///
/// Encode with lower quality (good for web thumbnails)
/// let webp_data = encode(image, 60.0)?;
pub fn encode(img: DynamicImage, quality: f32) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    // Convert the image to RGB8 format as required by the WebP encoder
    // This ensures consistent color representation regardless of input format
    let rgb = img.to_rgb8();

    // Extract image dimensions needed for encoder initialization
    let (width, height) = rgb.dimensions();

    // Initialize WebP encoder with the RGB pixel data and image dimensions
    let encoder = Encoder::from_rgb(&rgb, width, height);

    // Perform lossy WebP encoding with the specified quality level
    let webp = encoder.encode(quality);

    // Convert the encoded WebP data to a Vec<u8> for easy handling and storage
    Ok(webp.to_vec())
}