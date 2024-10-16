use clap::Parser;
use crossterm::tty::IsTty;

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

    // Get a reference to STDOUT
    let mut stdout = std::io::stdout();

    // Determine if we are in passthrough mode.
    // If the `passthrough` flag is set, or the terminal is not interactive...
    // we simply pipe the output through
    if args.passthrough || !stdout.is_tty() {
        let mut input = std::fs::File::open(&args.filename).expect("Failed to open the file");
        std::io::copy(&mut input, &mut stdout)?;
        return Ok(());
    }

    // Get the terminal width and height from crossterm
    let (_, height) = crossterm::terminal::size()?;

    // Initialize the Pager application
    let mut pager = pager::Pager::init(height as usize);

    // Run the Pager application
    pager.run(args)
}
