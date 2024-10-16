use clap::Parser;

mod cli;
mod pager;

/// The entry-point of the application
fn main() {
    let args = cli::Args::parse();
    match pager::Pager::run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
