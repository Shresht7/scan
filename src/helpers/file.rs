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
        let line_col = iter.next().unwrap_or("");
        let (row, col) = parse_row_and_col(line_col);
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

/// Parses a string line:col string into a tuple of numbers representing the row and col
pub fn parse_row_and_col(s: &str) -> (Option<usize>, Option<usize>) {
    let mut iter = s.split(":");
    let row = iter.next().and_then(|s| s.parse::<usize>().ok());
    let col = iter.next().and_then(|s| s.parse::<usize>().ok());
    (row, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_line_col_string() {
        assert_eq!((Some(5), Some(7)), parse_row_and_col("5:7"));
    }

    #[test]
    fn should_parse_line_alone() {
        assert_eq!((Some(5), None), parse_row_and_col("5"));
    }

    #[test]
    fn should_not_parse_anything_other_than_numbers() {
        assert_eq!((None, None), parse_row_and_col("row:col"));
    }
}
