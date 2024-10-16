use std::io::{BufRead, Seek};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEventKind},
    terminal, ExecutableCommand,
};

use crate::cli;

pub struct Pager {
    /// The index of the first-line to display
    scroll: u32,
    /// The max height of the page in the terminal
    page_height: usize,

    /// If true, exit the program
    exit: bool,
}

impl Pager {
    /// Instantiate the Pager application
    pub fn init() -> Pager {
        Self {
            scroll: 0,
            page_height: terminal::size().unwrap_or((120, 40)).1 as usize - 1,
            exit: false,
        }
    }

    /// The main application logic of the pager
    pub fn run(&mut self, args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        // Check if there are arguments are all valid
        if let Err(e) = self.validate_args(&args) {
            return Err(e); // If not, return early with the error
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

        // The main program loop. Break when the exit flag is set.
        while !self.exit {
            // Clear the screen and move the cursor to the top
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;
            stdout.execute(cursor::MoveTo(0, 0))?;

            // Since read_line will move the cursor each iteration, we need to seek back to the start
            file.seek(std::io::SeekFrom::Start(0))?; // Go back to the start of the file
            reader = std::io::BufReader::new(&file); // Reinitialize the BufReader

            // Skip all lines till the start of the tracked scroll position
            for _ in 0..self.scroll {
                let mut dummy = String::new();
                reader.read_line(&mut dummy)?;
            }
            // Read a page's worth of lines and print them
            for _ in 0..self.page_height {
                let mut line = String::new();
                if reader.read_line(&mut line)? == 0 {
                    break;
                }
                print!("{}", line)
            }

            // Handle key events before continuing to loop
            self.handle_events()?;
        }

        // Restore the terminal by exiting the Alternate Screen Buffer when we're done
        stdout.execute(terminal::LeaveAlternateScreen)?;

        Ok(())
    }

    /// Handle crossterm events like key-presses
    fn handle_events(&mut self) -> std::io::Result<()> {
        match crossterm::event::read()? {
            // It's important to check that the event is a key-press event as
            // crossterm also emits key-release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('k') => self.scroll(-1),
                    KeyCode::Down | KeyCode::Char('j') => self.scroll(1),
                    KeyCode::Esc | KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Scroll by the given offset. Positive numbers scroll down, whereas, negative numbers scroll up.
    fn scroll(&mut self, offset: i32) {
        // TODO: Need to bound the scroll values between the first and the last line
        self.scroll = self.scroll.saturating_add_signed(offset);
    }

    /// Set the exit flag to indicate that we exit the program loop
    fn exit(&mut self) {
        self.exit = true;
    }

    /// Validate that all arguments are as they should be
    fn validate_args(&self, args: &cli::Args) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the file exists...
        if !args.filename.exists() {
            // And return early with an error if it doesn't
            return Err(format!(
                "The provided file does not exist: {}",
                args.filename.to_string_lossy()
            )
            .into());
        }
        Ok(())
    }
}
