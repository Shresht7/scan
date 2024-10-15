// Traits
use clap::Parser;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    filename: String,
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
    println!("{}", args.filename);
    Ok(())
}
