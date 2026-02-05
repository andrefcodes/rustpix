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

//! AVIF encoding functionality.
//!
//! This module handles the conversion of decoded images into optimized AVIF format
//! with configurable quality using the ravif crate.

use image::DynamicImage;
use ravif::{Encoder, Img};
use rgb::RGBA8;
use std::error::Error;

/// Encode an image to AVIF format with specified quality.
///
/// # Arguments
/// * `img` - The decoded image as a `DynamicImage` (supports various pixel formats)
/// * `quality` - AVIF compression quality level (1-100, where 100 = highest quality)
///
/// # Returns
/// * `Ok(Vec<u8>)` - Successfully encoded AVIF image data as bytes
/// * `Err(Box<dyn Error>)` - Encoding failed due to invalid input or system constraints
///
/// # Quality Mapping
/// The quality parameter (1-100) is mapped to ravif's internal quality scale.
/// Higher values produce better quality but larger files.
pub fn encode(img: DynamicImage, quality: f32) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    // Convert to RGBA8 format for ravif encoder
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    // Convert image data to ravif's expected format
    let pixels: Vec<RGBA8> = rgba
        .pixels()
        .map(|p| RGBA8::new(p[0], p[1], p[2], p[3]))
        .collect();

    // Create an Img from the pixel data
    let img_data = Img::new(&pixels[..], width as usize, height as usize);

    // Map quality 1-100 to ravif's quality scale
    // ravif uses 0-100 where higher is better quality (same as our input)
    let ravif_quality = quality.clamp(1.0, 100.0);

    // Configure and run the encoder
    let encoder = Encoder::new()
        .with_quality(ravif_quality)
        .with_speed(6); // Speed 6 is a good balance (1=slowest/best, 10=fastest/worst)

    let result = encoder.encode_rgba(img_data)?;

    Ok(result.avif_file)
}
