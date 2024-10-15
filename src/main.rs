use clap::Parser;
use std::io::BufRead;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    #[clap(default_value = "-")]
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
    // Check if the file exists...
    if !args.filename.exists() {
        // And return early with an error if it doesn't
        return Err(format!(
            "The provided file does not exist: {}",
            args.filename.to_string_lossy()
        )
        .into());
    }

    // Open the file and instantiate a BufReader
    let file = std::fs::File::open(&args.filename).expect("Failed to open the file");
    let mut reader = std::io::BufReader::new(&file);

    // Read and print all the lines
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        print!("{}", line)
    }

    Ok(())
}
