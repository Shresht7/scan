use clap::Parser;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    pub filename: Option<std::path::PathBuf>,

    /// Pass the contents through without running the interactive Pager
    #[clap(short, long, aliases=["skip", "no-page"])]
    pub passthrough: bool,
}

impl Args {
    /// Validate that all arguments are as they should be
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the filename was passed in as an argument
        if let Some(filename) = &self.filename {
            // Check if the file exists...
            if !filename.exists() {
                // And return early with an error if it doesn't
                return Err(format!(
                    "The provided file does not exist: {}",
                    filename.to_string_lossy()
                )
                .into());
            }
        }
        Ok(())
    }
}
