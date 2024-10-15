use std::io::{BufRead, Seek};

use clap::Parser;
use crossterm::{cursor, terminal, ExecutableCommand};

mod cli;

/// The entry-point of the application
fn main() {
    let args = cli::Args::parse();
    match run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// The main logic of the application
fn run(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut file = std::fs::File::open(&args.filename).expect("Failed to open the file");
    let mut reader = std::io::BufReader::new(&file);

    // Prepare stdout by entering the Alternate Screen Buffer,
    // clearing the terminal and moving the cursor to the 0, 0 position
    let mut stdout = std::io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    // Track the position of the current line
    let mut current_line = 0;
    // The max height of the page in the terminal
    let page_height = terminal::size()?.1 as usize - 1;

    loop {
        // Clear the screen and move the cursor to the top
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // Since read_line will move the cursor each iteration, we need to seek back to the start
        file.seek(std::io::SeekFrom::Start(0))?; // Go back to the start of the file
        reader = std::io::BufReader::new(&file); // Reinitialize the BufReader

        // Skip all lines till the start of the tracked current_line
        for _ in 0..current_line {
            let mut dummy = String::new();
            reader.read_line(&mut dummy)?;
        }
        // Read a page's worth of lines and print them
        for _ in 0..page_height {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            print!("{}", line)
        }

        // ! FIXME: This currently run indefinitely! Handle exit event using crossterm
    }

    // Restore the terminal by exiting the Alternate Screen Buffer when we're done
    stdout.execute(terminal::LeaveAlternateScreen)?;

    Ok(())
}
