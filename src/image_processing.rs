// src/image_processing.rs
use image::{io::Reader as ImageReader, DynamicImage};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use webp::Encoder;

pub fn process_image(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Open the image and strip EXIF data
    let img = ImageReader::open(file_path)?.decode()?;

    // Convert to WebP format with 75% quality using the webp crate
    let webp_data = encode_webp(img, 75.0)?;

    // Generate a new UUID-based file name
    let uuid = Uuid::new_v4();
    let new_file_name = format!("{}.webp", uuid);
    let new_file_path = file_path.with_file_name(new_file_name);

    // Save the new file
    let mut output_file = File::create(&new_file_path)?;
    output_file.write_all(&webp_data)?;

    // Delete the original file
    fs::remove_file(file_path)?;

    println!("Processed: {:?}", new_file_path);
    Ok(())
}

fn encode_webp(img: DynamicImage, quality: f32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Convert image to RGB format for WebP encoding
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let encoder = Encoder::from_rgb(&rgb, width, height);
    let webp = encoder.encode(quality);
    Ok(webp.to_vec())
}