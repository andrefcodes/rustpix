# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.4.0-alpha.1] - 2026-02-05

### Added
- **AVIF output format** - New `-f/--format` flag to choose output format (webp or avif)
- **AVIF input support** - Can now read and convert AVIF images via libheif
- **WebP input support** - Can now read and convert WebP images
- **Image cropping** - New `-c/--crop <width> <height>` flag to crop output (centered)
- **Short ID flag** - `-S` now works as short form for `--short-id`

### Changed
- **Dry-run flag** - Changed from `-n` to `-d/--dry-run`
- Version bumped to 0.4.0-alpha.1
- Updated dependencies:
  - Added `ravif` 0.13 for AVIF encoding
  - Added `rgb` 0.8 for pixel types
  - Enabled `avif` feature in `image` crate

### Removed
- Removed ROADMAP.md (features now tracked in GitHub issues)

## [0.3.0-alpha.1] - 2025-01-15

### ⚠️ Breaking Changes
- HEIF support now requires system libheif >= 1.17 (embedded build removed)
- UUID v4 replaced with UUID v7 (timestamp-based, sortable) for output filenames

### Added
- **Alpha status signaling** - Clear warnings in CLI output about alpha status
- **Progress bar** - Visual indicator for batch operations using `indicatif`
- **Compression statistics** - New `-s/--stats` flag shows before/after sizes
- **Max size targeting** - New `-m/--max-size` flag with auto quality adjustment
  - Supports human-readable sizes: `500KB`, `2MB`, etc.
  - Uses binary search for optimal quality
- **SVG support** - SVG vector graphics rasterization via `resvg` crate (pure Rust)
- **GIF detection** - Basic animated GIF detection (first frame conversion)
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
  - `libheif-rs` 2.6.1
  - `resvg` 0.46.0
  - `indicatif` 0.18.3
  - `walkdir` 2.5
- Improved CLI help with examples
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