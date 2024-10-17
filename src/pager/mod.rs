use std::io::BufRead;

use crossterm::{cursor, terminal, ExecutableCommand};

use crate::{cli, helpers};

mod events;

pub struct Pager {
    /// The collection of buffered lines
    lines: Vec<String>,

    /// The index of the first-line to display in the viewport
    scroll_row: usize,
    /// The index of the first-column to display in the viewport
    scroll_col: usize,
    /// The max height of the page in the terminal
    height: usize,
    /// The max width of the page in the terminal
    width: usize,

    /// If true, exit the program
    exit: bool,
}

impl Pager {
    /// Instantiate the Pager application
    pub fn init(size: (u16, u16)) -> Pager {
        Self {
            lines: Vec::new(),
            scroll_row: 0,
            scroll_col: 0,
            height: size.1 as usize,
            width: size.0 as usize,
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
            // Buffer more lines as needed based on the self.scroll and self.page_height variables
            self.buffer_lines(&mut reader)?;

            // Render the pager's view
            self.render(&mut stdout)?;

            // Handle key events before continuing to loop
            self.handle_events()?;
        }

        // Restore the terminal by exiting the Alternate Screen Buffer when we're done
        stdout.execute(terminal::LeaveAlternateScreen)?;

        Ok(())
    }

    /// Render the Pager's view
    fn render(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        // Clear the screen and move the cursor to the top
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        // Iterate over the lines in the viewport ...
        for l in &self.lines[self.view_start()..self.view_end()] {
            // The final formatted line to be printed to the terminal
            let mut line = String::from(l);

            // Clip the string for horizontal scroll
            if self.scroll_col > 0 {
                line = match l.split_at_checked(self.scroll_col) {
                    Some((_, x)) => String::from(x),
                    None => String::new(),
                }
            }

            // Truncate the line to fit in the page width
            line.truncate(self.width);

            // Print out the formatted line
            println!("{}", line);
        }

        Ok(())
    }

    /// Buffer lines from the reader as needed
    fn buffer_lines(&mut self, reader: &mut Box<dyn BufRead>) -> std::io::Result<()> {
        for line in reader.lines() {
            self.lines.push(line?);
            // Read up to the viewport's end + one more page
            if self.lines.len() > self.view_end() + self.height {
                break;
            }
        }
        Ok(())
    }

    // HELPER FUNCTIONS
    // ----------------

    /// The start of the viewport. Index of the first visible line
    fn view_start(&self) -> usize {
        self.scroll_row
    }

    /// The end of the viewport. Index of the last visible line
    fn view_end(&self) -> usize {
        self.scroll_row + self.height - 1
    }

    /// Set the exit flag to indicate that we need to exit the program
    fn exit(&mut self) {
        self.exit = true;
    }
}
