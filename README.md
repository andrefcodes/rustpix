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

clone this repo

`git clone https://github.com/a-franca/rustpix.git ~/.local/share/rustpix`,

`cd ~/.local/share/rustpix`

`cargo build --release`

**Dependencies:**

This program is built in Rust. For instructions on how to install it, check [Rust's offical webpage](https://www.rust-lang.org/tools/install).
Depending on your system you may need to install additional packages like `libexif-dev` and `pkg-config`.

## Add the directory to your system's PATH

`echo 'export PATH="$PATH:$HOME/.local/share/rustpix/target/release"' >> ~/.bashrc`

`source ~/.bashrc`

## Usage

`rustpix file.ext` for 1 file

`rustpix file1.jpeg file2.png file3.jpeg` for multiple files

`rustpix *.ext` for multiple files of the same extension

`rustpix *` for every image inside the directory