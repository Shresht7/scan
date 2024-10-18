use clap::Parser;
use crossterm::{
    cursor,
    style::{style, Stylize},
    terminal,
    tty::IsTty,
    ExecutableCommand,
};

mod cli;
mod helpers;
mod pager;

/// The entry-point of the application
fn main() {
    // Parse the command-line arguments
    let args = cli::Args::parse();
    // Run the main logic with the given command-line arguments
    match run(&args) {
        Err(e) => {
            print_error(e);
            std::process::exit(1)
        }
        Ok(_) => std::process::exit(0),
    }
}

/// Run the main logic of the application
fn run(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    // Get a reference to STDOUT
    let mut stdout = std::io::stdout();

    // Get a reference to the reader
    let mut reader = helpers::get_reader(&args.file)?;

    // Determine if we are in passthrough mode.
    // If the `passthrough` flag is set, or the terminal is not interactive...
    // we simply pipe the output through
    if args.passthrough || !stdout.is_tty() {
        std::io::copy(&mut reader, &mut stdout)?;
        return Ok(());
    }

    // Get the terminal width and height from crossterm
    let size = crossterm::terminal::size()?;

    // Initialize the Pager application
    let mut pager = pager::Pager::init(size);

    // Set options
    pager
        .with_line_numbers(args.show_line_numbers)
        .with_borders(args.borders)
        .all(args.all);

    // Set scroll offsets
    if let Some(file) = &args.file {
        pager.with_offset(file.row, file.col);
    }

    // Setup the terminal before running the Pager application
    setup(&mut stdout)?;

    // Run the Pager application
    pager.run(reader, &mut stdout)?;

    // Cleanup the terminal when the Pager application exits
    cleanup(&mut stdout)?;

    Ok(())
}

/// Prepare stdout by entering the Alternate Screen Buffer,
/// clearing the terminal and moving the cursor to the 0, 0 position
fn setup(stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    // Create a custom hook to handle graceful cleanup of the terminal when panicking
    let original_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut stdout = std::io::stdout();
        // Intentionally ignore errors here since we're already in a panic!
        let _ = cleanup(&mut stdout);
        original_panic(info);
    }));

    Ok(())
}

/// Restore the terminal by exiting the Alternate Screen Buffer when we're done
fn cleanup(stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(terminal::LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;
    Ok(())
}

/// Prints the human friendly error message
fn print_error(e: Box<dyn std::error::Error>) {
    let message = style(format!("Error: {e}")).red();
    if std::io::stderr().is_tty() {
        eprintln!("{message}")
    }
}
