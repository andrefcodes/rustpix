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

//! Image format handlers for rustpix.
//!
//! This module organizes and exports all image format-specific functionality.
//! It serves as the central registry for format decoders and encoders.

/// WebP format handling - provides encoding functionality for the primary output format
pub mod webp;

/// HEIC/HEIF format handling - provides decoding functionality for Apple's image formats
#[cfg(feature = "heif")]
pub mod heif;

/// SVG format handling - provides rasterization for vector graphics
#[cfg(feature = "svg")]
pub mod svg;
