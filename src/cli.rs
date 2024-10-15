use clap::Parser;

/// The command-line arguments
#[derive(Parser)]
pub struct Args {
    /// The file to view
    #[clap(default_value = "-")]
    pub filename: std::path::PathBuf,
}
