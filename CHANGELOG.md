# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.3.0-alpha.1] - 2025-01-15

### ⚠️ Breaking Changes
- HEIF feature now requires system libheif >= 1.17 (embedded build removed)
- UUID v4 replaced with UUID v7 (timestamp-based, sortable) for output filenames

### Added
- **Alpha status signaling** - Clear warnings in CLI output about alpha status
- **Progress bar** - Visual indicator for batch operations using `indicatif`
- **Compression statistics** - New `-s/--stats` flag shows before/after sizes
- **Max size targeting** - New `-m/--max-size` flag with auto quality adjustment
  - Supports human-readable sizes: `500KB`, `2MB`, etc.
  - Uses binary search for optimal quality
- **SVG support** - New `svg` feature for vector graphics rasterization
  - Pure Rust via `resvg` crate (no system dependencies)
- **GIF detection** - Basic animated GIF detection (first frame conversion)
- **Build features system** - Modular feature flags:
  - `default`: All features enabled (HEIF + SVG)
  - `svg`: SVG rasterization (pure Rust, no system deps)
  - `heif`: HEIC/HEIF support (requires system libheif >= 1.17)
- **Summary statistics** - Batch processing shows total savings
- **Recursive directory processing** - New `-r/--recursive` flag to process directories
- **Dry-run mode** - New `-n/--dry-run` flag to preview operations
- **Verbosity controls** - New `-V/--verbose` and `-Q/--quiet` flags
- **Short UUID option** - New `--short-id` flag for 8-character filenames

### Changed
- Version bumped to 0.3.0-alpha.1
- Added `publish = false` to prevent accidental crates.io publication
- Updated dependencies to latest versions:
  - `image` 0.25.9
  - `uuid` 1.19.0 (now using v7 instead of v4)
  - `rayon` 1.11
  - `webp` 0.3.1
  - `libheif-rs` 1.0.0 (for 1.17 compatibility)
  - `indicatif` 0.18.3 (new)
  - `walkdir` 2.5 (new)
- Improved CLI help with examples and feature-aware format list
- Enhanced error messages with visual indicators (✓/✗)

### Fixed
- Better error handling throughout the codebase
- Consistent error types with `Send + Sync`

## [0.2.0-alpha.1] - 2025-06-12

### Added
- HEIC/HEIF format support
  - Added the ability to detect and convert HEIC/HEIF images
  - Uses libheif-rs for high-quality conversion
  - Detects HEIC/HEIF files by extension and file signature

### Changed
- Complete code restructuring into a modular architecture
  - Split functionality into logical modules:
    - `cli.rs`: Command-line interface parsing
    - `processing.rs`: Core image processing
    - `formats/webp.rs`: WebP encoding
    - `formats/heif.rs`: HEIC/HEIF decoding
  - Improved maintainability and extensibility
- Enhanced documentation throughout the codebase
- Updated usage instructions to mention HEIC/HEIF support

### Fixed
- Improved error handling throughout the application

## [0.1.0-alpha.1] - 2024-12-22

### Added
- Initial release
- Basic image format support (PNG, JPEG, GIF, BMP, ICO, TIFF, AVIF)
- WebP conversion with quality control
- Parallel processing using rayon
- CLI parameters for customizing operation