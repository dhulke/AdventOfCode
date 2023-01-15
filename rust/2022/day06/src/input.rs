use std::fs::File;
use std::io;
use std::path::Path;
use std::io::prelude::*;

pub fn get_file_lines(file_name: impl AsRef<Path>) -> io::Result<impl Iterator<Item=char>> {
    Ok(io::BufReader::new(File::open(file_name)?)
        .bytes()
        .map(|byte| byte.expect("Error retrieving line from file") as char ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_three_lines_from_file() {
        let three_lines_file_name = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/characters.txt");
        let mut line_iter = get_file_lines(three_lines_file_name)
            .expect("File exists for testing");

        assert_eq!(line_iter.next(), Some('f'));
        assert_eq!(line_iter.next(), Some('i'));
        assert_eq!(line_iter.next(), Some('r'));
    }
}