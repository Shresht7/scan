use std::io::Read;

// Traits
use clap::Parser;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    filename: std::path::PathBuf,
}

/// The entry-point of the application
fn main() {
    let args = Args::parse();
    match run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// The main logic of the application
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(&args.filename).expect("Failed to open the file");
    let mut contents: String = String::new();
    let res = file.read_to_string(&mut contents)?;
    println!("{}", contents);
    println!("Read: {}", res);
    Ok(())
}
