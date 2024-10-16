use clap::Parser;

mod cli;
mod pager;

/// The entry-point of the application
fn main() {
    // Parse the command-line arguments
    let args = cli::Args::parse();

    // Validate the command-line arguments
    if let Err(e) = args.validate() {
        eprintln!("Argument Parse Error: {}", e);
        std::process::exit(1);
    }

    // Run the Pager application
    match pager::Pager::init().run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
