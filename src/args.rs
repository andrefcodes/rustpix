// src/args.rs
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: rustpix <file1> [file2 ...] or rustpix *.<pattern> or rustpix *");
        std::process::exit(1);
    }

    // Check for version flag
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("rustpix version {}", VERSION);
        std::process::exit(0);
    }

    args
}