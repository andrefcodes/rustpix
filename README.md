[![Cargo Build & Test](https://github.com/andrefcodes/rustpix/actions/workflows/ci.yml/badge.svg)](https://github.com/andrefcodes/rustpix/actions/workflows/ci.yml) [![Rust-clippy analyze](https://github.com/andrefcodes/rustpix/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/andrefcodes/rustpix/actions/workflows/rust-clippy.yml) [![Release](https://github.com/andrefcodes/rustpix/actions/workflows/release.yml/badge.svg)](https://github.com/andrefcodes/rustpix/actions/workflows/release.yml)

# Rustpix

## Overview

**Rustpix** is a command-line tool that optimizes image files for the Web. It offers parallel processing for multiple files and includes some other options.

## Features:

- Converts images of any popular format like PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, and AVIF to an optimized Webp image;
- Strips exif information from images;
- Processes multiple files concurrently using [Rayon](https://github.com/rayon-rs/rayon) for improved performance;
- Keep Originals: Optional flag (-k, --keep-original) allows preserving original files;
- Supports custom naming for output files (Defaults to UUIDv4.webp name);
- Allows for quality selection (Defaults to 75% if none is provided).

New functionalities on the way, like HEIC/HEIF support...

## Download

Download the binary for your system at the [Releases Page](https://github.com/andrefcodes/rustpix/releases) or [Build the Source Code](https://github.com/andrefcodes/rustpix#build) yourself.

Also, see how to [add the directory to your system's path](https://github.com/andrefcodes/rustpix#add-the-directory-to-your-systems-path).

**Note** that while the binary was compiled for other platforms, it hasn't been tested on them, but linux x86_x64.

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


## Build

1. Clone this repo

```bash
git clone https://github.com/andrefcodes/rustpix.git ~/.local/share/rustpix
```

```bash
cd ~/.local/share/rustpix
```

2. Build

```bash
cargo build --release
```

**Dependencies:**

This program is built in Rust. For instructions on how to install it, check [Rust's offical webpage](https://www.rust-lang.org/tools/install).
Depending on your system you may need to install additional packages like `libexif-dev` and `pkg-config`.

## Add the directory to your system's PATH

```bash
echo 'export PATH="$PATH:$HOME/.local/share/rustpix/target/release"' >> ~/.bashrc
```

Replace path if you place your binary in a different directory.
The same is valid for `.bashrc` if you're working with zsh or fish.

```bash
source ~/.bashrc
```

## License

This project is distributed under the terms of both the MIT license and the Apache License (Version 2.0). See [LICENSE-MIT](https://github.com/andrefcodes/rustpix/blob/main/LICENSE-MIT) and [LICENSE-APACHE](https://github.com/andrefcodes/rustpix/blob/main/LICENSE-APACHE) for details.
