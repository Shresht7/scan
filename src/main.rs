use clap::Parser;
use crossterm::{cursor, terminal, tty::IsTty, ExecutableCommand};

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
            helpers::print_error(e);
            std::process::exit(1)
        }
        Ok(_) => std::process::exit(0),
    }
}

/// Run the main logic of the application
fn run(args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    // Get a reference to the standard output
    let mut stdout = std::io::stdout();

    // Instantiate a reader to read from. This can be a file or standard input
    let mut reader = helpers::get_reader(&args.file)?;

    // Determine if we are in passthrough mode.
    // If the `passthrough` flag is set, or the terminal is not interactive...
    // we simply pipe the output through
    if args.passthrough || !stdout.is_tty() {
        std::io::copy(&mut reader, &mut stdout)?;
        return Ok(());
    }

    // Initialize the Pager application
    let size = crossterm::terminal::size()?;
    let mut pager = pager::Pager::init(size);

    // Set configuration options
    pager
        .with_line_numbers(args.show_line_numbers)
        .with_borders(args.show_borders)
        .all(args.all);

    if let Some(file) = &args.file {
        pager.with_offset(file.row, file.col);
    }

    // Setup the terminal before running the application
    setup(&mut stdout)?;

    // Run the Pager application
    pager.run(reader, &mut stdout)?;

    // Cleanup the terminal after the Pager application exits
    cleanup(&mut stdout)?;

    Ok(())
}

/// Prepares the terminal for the application.
/// Switches to the Alternate Screen Buffer and clears the screen.
/// Also moves the cursor to the top and hides it.
/// Registers a panic-hook to automatically call the `cleanup` function
fn setup(stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(cursor::Hide)?;

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

/// Restore the terminal by exiting the Alternate Screen Buffer when we're done. Also re-enables the cursor.
fn cleanup(stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(terminal::LeaveAlternateScreen)?;
    stdout.execute(cursor::Show)?;
    Ok(())
}
