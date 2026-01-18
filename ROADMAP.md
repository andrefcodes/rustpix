# Rustpix Feature Possibilities

This document outlines possible features and improvements that could be added to rustpix in the future. These are organized by category rather than representing committed development plans.

## ✅ Completed (v0.3.0-alpha.1)
- **Progress bar** - Visual indicator for batch operations
- **Compression statistics** - Show before/after size comparison (`-s` flag)
- **Max size flag** - Target a specific maximum file size with automatic quality adjustment (`-m` flag)
- **SVG support** - Rasterization of vector graphics to WebP (via `svg` feature)
- **GIF support** - Basic handling (first frame to static WebP)
- **Recursive directory processing** - Process all images in subdirectories (`-r` flag)
- **Dry-run mode** - Preview what would be converted without making changes (`-n` flag)
- **Verbose/quiet modes** - Control output verbosity (`-V`/`-Q` flags)

## Future Possibilities

### User Experience
- **Config file support** - Save preferred settings

### Core Features
- **Animated WebP output** - Full animation support for GIF/APNG
- **Resize options** - Scale images during conversion
- **Crop options** - Crop images during conversion

### Performance
- **Memory limit** - Cap memory usage for large images
- **Streaming processing** - Process very large images without loading fully into memory

## Contributing Features

If you're interested in implementing any of these features or have other ideas, please open an issue on [GitHub](https://github.com/andrefcodes/rustpix/issues) or submit a pull request. See [CONTRIBUTING.md](CONTRIBUTING.md) for more information on how to contribute.

## Feature Requests

Have an idea for a feature that's not listed here? Here's how to suggest it:

1. Open an issue on [GitHub](https://github.com/andrefcodes/rustpix/issues) with the title "Feature Request: [Your Feature]"
2. Describe the feature and why it would be valuable
3. If possible, provide examples of how it might work

All feature requests are welcome and will be considered based on how they align with the project's goals.