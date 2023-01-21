use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_file_lines(file_name: impl AsRef<Path>) -> io::Result<impl Iterator<Item=String>> {
    Ok(io::BufReader::new(File::open(file_name)?)
        .lines()
        .map(|x| x.expect("Error retrieving line from file")))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_three_lines_from_file() {
        let three_lines_file_name = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/three_lines.txt");
        let mut line_iter = get_file_lines(three_lines_file_name).expect("File exists for testing");

        assert_eq!(line_iter.next(), Some("first line".to_string()));
        assert_eq!(line_iter.next(), Some("second line".to_string()));
        assert_eq!(line_iter.next(), Some("third line".to_string()));
    }
}