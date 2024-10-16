use clap::Parser;

mod cli;
mod pager;

/// The entry-point of the application
fn main() {
    // Parse the command-line arguments
    let args = cli::Args::parse();
    // Run the main logic with the given command-line arguments
    match run(&args) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
        Ok(_) => std::process::exit(0),
    }
}

/// Run the main logic of the application
fn run(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    // Validate the command-line arguments
    args.validate()?;
    // Run the Pager application
    pager::Pager::init().run(args)
}
