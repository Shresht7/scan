use std::str::FromStr;

#[derive(Clone)]
pub struct File {
    filename: String,
    pub row: Option<usize>,
    pub col: Option<usize>,
}

impl FromStr for File {
    type Err = String; // Define the error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(":");
        let filename = iter.next().unwrap_or(s).to_string();
        let row = iter.next().and_then(|s| s.parse::<usize>().ok());
        let col = iter.next().and_then(|s| s.parse::<usize>().ok());
        Ok(Self { filename, row, col })
    }
}

/// Returns a BufReader. If a filepath is specified, returns a BufReader for the File,
/// otherwise, returns a BufReader for STDIN.
pub fn get_reader(
    file: &Option<File>,
) -> Result<Box<dyn std::io::BufRead>, Box<dyn std::error::Error>> {
    let reader: Box<dyn std::io::BufRead> = if let Some(file) = &file {
        let filepath = std::path::Path::new(&file.filename);
        if !filepath.exists() {
            return Err(format!("The provided file does not exist: {}", file.filename).into());
        }
        let file = std::fs::File::open(filepath)?;
        Box::new(std::io::BufReader::new(file))
    } else {
        Box::new(std::io::stdin().lock())
    };
    Ok(reader)
}