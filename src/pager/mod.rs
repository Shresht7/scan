use std::io::BufRead;

use crossterm::{cursor, terminal, ExecutableCommand};

use crate::{cli, helpers};

mod events;

pub struct Pager {
    /// The collection of buffered lines
    lines: Vec<String>,

    /// The index of the first-line to display
    scroll: usize,
    /// The max height of the page in the terminal
    page_height: usize,

    /// If true, exit the program
    exit: bool,
}

impl Pager {
    /// Instantiate the Pager application
    pub fn init(height: usize) -> Pager {
        Self {
            lines: Vec::new(),
            scroll: 0,
            page_height: height,
            exit: false,
        }
    }

    /// The main application logic of the pager
    pub fn run(&mut self, args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        // Read all the lines at once
        let mut reader = helpers::get_reader(&args.filename)?;

        // Buffer initial set of lines
        self.buffer_lines(&mut reader)?;

        // Prepare stdout by entering the Alternate Screen Buffer,
        // clearing the terminal and moving the cursor to the 0, 0 position
        let mut stdout = std::io::stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // The main program loop. Break when the exit flag is set.
        while !self.exit {
            // Clear the screen and move the cursor to the top
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            stdout.execute(cursor::MoveTo(0, 0))?;

            // Read a page's worth of lines and print them
            for line in &self.lines[self.scroll..(self.scroll + self.page_height - 1)] {
                println!("{}", line)
            }

            // Handle key events before continuing to loop
            self.handle_events()?;

            // Buffer more lines as needed based on the self.scroll and self.page_height variables
            self.buffer_lines(&mut reader)?;
        }

        // Restore the terminal by exiting the Alternate Screen Buffer when we're done
        stdout.execute(terminal::LeaveAlternateScreen)?;

        Ok(())
    }

    /// Buffer lines from the reader as needed
    fn buffer_lines(&mut self, reader: &mut Box<dyn BufRead>) -> std::io::Result<()> {
        for line in reader.lines() {
            self.lines.push(line?);
            if self.lines.len() >= self.scroll + self.page_height {
                break;
            }
        }
        Ok(())
    }

    /// Set the exit flag to indicate that we need to exit the program
    fn exit(&mut self) {
        self.exit = true;
    }
}
