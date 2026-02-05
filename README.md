# rustpix

> ⚠️ **ALPHA SOFTWARE** - This project is in early development. Features may be incomplete, unstable, or change without notice.

## Overview

**Rustpix** is a command-line tool that optimizes image files for the Web.  
It offers parallel processing for multiple files and includes several customization options.

### Why Rustpix?

Unlike heavyweight tools like FFmpeg or ImageMagick, Rustpix is:
- **Performance-oriented** - Built with Rust for efficiency and parallel processing
- **Focused on web optimization** - Specializes in creating optimized WebP files, not a general-purpose image tool
- **Privacy-conscious** - Always strips metadata (EXIF, etc.) to protect privacy
- **Simple to use** - Minimal command-line interface with sensible defaults
- **Purposely minimal** - Designed to do one thing well: optimize images for the web

## Features

- **Converts images** of popular formats (PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF) to optimized WebP or AVIF
- **SVG support** - Rasterize vector graphics to WebP or AVIF
- **HEIC/HEIF support** - Handle Apple device images (requires system libheif >= 1.17)
- **Multiple output formats** - Choose between WebP (default) or AVIF with `-f` flag
- **Image cropping** - Crop output to specific dimensions with `-c` flag
- **Strips all metadata** for privacy protection
- **Parallel processing** using [Rayon](https://github.com/rayon-rs/rayon) for batch operations
- **Progress bar** - Visual indicator for batch operations
- **Compression statistics** - Show before/after size comparison with `-s` flag
- **Max size targeting** - Automatically adjust quality to meet file size targets with `-m` flag
- **Quality selection** - Customizable quality (defaults to 75%)
- **Recursive processing** - Process entire directory trees with `-r` flag
- **Dry-run mode** - Preview operations without making changes with `-d` flag
- **Verbosity controls** - Quiet mode (`-Q`) or verbose mode (`-V`)
- **Short IDs** - Use 8-character UUIDs for output filenames with `-S` or `--short-id`

## Usage

1. Convert a single file and delete the original:

```bash
rustpix file1.jpeg
```

2. Convert multiple files in parallel and delete originals:

```bash
rustpix file1.jpeg file2.png file3.bmp
```

3. Convert only files from a specific extension

```bash
rustpix *.jpeg
```

4. Convert all files whihin a directory (only compatible image files are processed)

```bash
rustpix *
```

5. Convert files while keeping the originals:

```bash
rustpix file1.jpeg file2.png -k
```

6. Specify a custom output filename base

```bash
rustpix file1.jpeg -o my_cool_name
```
7. Specify custom output filename base for multiple files:

```bash
rustpix file1.jpeg file2.png file3.bmp -o my_cool_name
```
This will output my_cool_name1.webp, my_cool_name2.webp, and my_cool_name3.webp

8. Specify output image quality

```bash
rustpix file1.jpeg -q 60
```

9. Show compression statistics

```bash
rustpix *.png -s
```

10. Target a maximum file size (automatically adjusts quality)

```bash
rustpix photo.jpg -m 500KB
```

11. Combine options: keep originals, show stats, target max size

```bash
rustpix *.png -k -s -m 200KB
```

12. Process entire directory recursively

```bash
rustpix ./images -r
```

13. Preview what would be converted (dry-run)

```bash
rustpix ./images -r -d
```

14. Use short 8-character filenames

```bash
rustpix *.png -S
```

15. Silent batch processing (only show errors)

```bash
rustpix ./images -r -Q
```

16. Convert to AVIF format

```bash
rustpix image.png -f avif
```

17. Convert WebP to AVIF

```bash
rustpix photo.webp -f avif
```

18. Crop output to specific dimensions (centered)

```bash
rustpix photo.jpg -c 650 985
```

19. Combine crop with format and quality options

```bash
rustpix photo.png -c 1200 800 -f avif -q 85 -k
```

## Installation

### Option 1: Download Pre-built Binary (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/andrefcodes/rustpix/releases):

| Platform | Download |
|----------|----------|
| Linux x86_64 | `rustpix-*-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `rustpix-*-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Apple Silicon | `rustpix-*-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `rustpix-*-x86_64-pc-windows-msvc.zip` |
*Note: Pre-built binaries for macOS Intel (x86_64) are no longer provided. If you need a binary for Intel Macs, please build from source as described below.*

```bash
# Example for Linux x86_64:
curl -LO https://github.com/andrefcodes/rustpix/releases/latest/download/rustpix-x86_64-unknown-linux-gnu.tar.gz
tar -xzf rustpix-*.tar.gz
sudo mv rustpix /usr/local/bin/
```

### Option 2: Build from Source

#### Requirements

- **Rust** - [Installation instructions](https://www.rust-lang.org/tools/install)
- **libheif-dev** >= 1.17 and **pkg-config**

#### Build Steps

1. Clone this repo

```bash
git clone https://github.com/andrefcodes/rustpix ~/.local/share/rustpix
cd ~/.local/share/rustpix
```

2. Build

```bash
cargo build --release
```

3. Install (optional)

```bash
cargo install --path .
# Or copy the binary manually:
cp target/release/rustpix ~/.local/bin/
```

## Contributing Features

If you're interested in implementing any of these features or have other ideas, please open an issue on [GitHub](https://github.com/andrefcodes/rustpix/issues) or submit a pull request. See [CONTRIBUTING.md](CONTRIBUTING.md) for more information on how to contribute.

## License

Rustpix - A Command-Line Tool for Image Optimization for the Web.  
Copyright (C) 2025 Andre Franca

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.