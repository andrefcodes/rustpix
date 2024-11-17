# rustpix

This is a light and simple program that automatically optimizes images for web.

Basically:

- It receives an image or multiple images of any popular format like jpeg, png, ico, svg, and gif as input
- remove all exif information from the file(s)
- convert the file(s) to a webp lossy file(s) up to 75% quality
- rename each file to an uuid gen 4 name.webp
- output the new file(s) to the same directory
- delete the original file(s)

In the future, I'll implement new functionalities.


## Build

clone this repo, `cd rustpix`, then

`cargo build --release`

## Usage

`./target/release/rustpix file.ext` for 1 file

`./target/release/rustpix file1.jpeg file2.png file3.jpeg` for multiple files

`./target/release/rustpix *.ext` for multiple files of the same extension
