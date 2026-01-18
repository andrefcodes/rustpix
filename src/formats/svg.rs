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

//! SVG rasterization functionality.
//!
//! This module converts vector graphics (SVG) to raster images that can then be
//! encoded to WebP format.

use image::{DynamicImage, RgbaImage};
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{Options, Tree};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Rasterize an SVG file to a DynamicImage
///
/// # Arguments
/// * `path` - Path to the SVG file
///
/// # Returns
/// * `Ok(DynamicImage)` - Successfully rasterized image
/// * `Err(Box<dyn Error>)` - Rasterization failed
///
/// # Notes
/// The SVG is rendered at its natural size. For very small SVGs,
/// a minimum size is enforced to ensure readable output.
pub fn rasterize(path: &Path) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    // Read the SVG file content
    let svg_data = fs::read(path)?;

    // Parse the SVG into a usvg tree
    let options = Options::default();
    let tree = Tree::from_data(&svg_data, &options)?;

    // Get the SVG's natural size
    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;

    // Ensure minimum dimensions for very small SVGs
    let min_size = 64u32;
    let (render_width, render_height) = if width < min_size || height < min_size {
        let scale = (min_size as f32 / width.min(height) as f32).max(1.0);
        (
            (width as f32 * scale).ceil() as u32,
            (height as f32 * scale).ceil() as u32,
        )
    } else {
        (width, height)
    };

    // Create a pixmap for rendering
    let mut pixmap = Pixmap::new(render_width, render_height)
        .ok_or("Failed to create pixmap for SVG rendering")?;

    // Calculate transform if we need to scale
    let transform = if render_width != width || render_height != height {
        resvg::tiny_skia::Transform::from_scale(
            render_width as f32 / width as f32,
            render_height as f32 / height as f32,
        )
    } else {
        resvg::tiny_skia::Transform::identity()
    };

    // Render the SVG
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Convert pixmap to image::RgbaImage
    let rgba_image = RgbaImage::from_raw(render_width, render_height, pixmap.take())
        .ok_or("Failed to create RGBA image from rendered SVG")?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}
