//! Module for file input/output operations.

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Reads lines from a file and returns them as a `Vec<String>`.
///
/// # Arguments
///
/// * `file_path` - A string slice representing the file path.
///
/// # Returns
///
/// An `io::Result` containing a vector of strings, each representing a line from the file.
pub fn read_lines_as_vec<P>(file_path: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    println!("Reading file: {}", file_path.as_ref().display());

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;
    use super::*;

    fn create_temp_file_with_content(contents: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap_or_else(|_| panic!("Could not create temp file"));
        file.write_all(contents).unwrap_or_else(|_| panic!("Could not write to temp file"));
        file
    }

    #[test]
    fn test_read_lines_as_vec_valid_file() {
        let temp_file = create_temp_file_with_content(b"Line 1\nLine 2");
        let path = temp_file.path();

        let result = read_lines_as_vec(path);
        assert!(result.is_ok());

        let lines = result.unwrap();
        assert_eq!(lines, vec!["Line 1", "Line 2"]);

        temp_file.close().unwrap();
    }

    #[test]
    fn test_read_lines_as_vec_invalid_file() {
        let path = Path::new("nonexistent_file.txt");
        let result = read_lines_as_vec(path);
        assert!(result.is_err());
    }
}
