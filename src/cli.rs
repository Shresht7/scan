use clap::Parser;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    #[clap(default_value = "-")]
    pub filename: std::path::PathBuf,
}

impl Args {
    /// Validate that all arguments are as they should be
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the file exists...
        if !self.filename.exists() {
            // And return early with an error if it doesn't
            return Err(format!(
                "The provided file does not exist: {}",
                self.filename.to_string_lossy()
            )
            .into());
        }
        Ok(())
    }
}
