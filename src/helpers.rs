/// Returns a BufReader. If a filepath is specified, returns a BufReader for the File,
/// otherwise, returns a BufReader for STDIN.
pub fn get_reader(
    filepath: &Option<std::path::PathBuf>,
) -> Result<Box<dyn std::io::BufRead>, Box<dyn std::error::Error>> {
    let reader: Box<dyn std::io::BufRead> = if let Some(filepath) = &filepath {
        let file = std::fs::File::open(filepath)?;
        Box::new(std::io::BufReader::new(file))
    } else {
        Box::new(std::io::stdin().lock())
    };
    Ok(reader)
}
