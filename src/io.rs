use std::{
    fs::read_to_string,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn from_file(filename: impl AsRef<Path>) -> String {
    read_to_string(filename).expect("No such file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_from_file() {
        let lines = lines_from_file("README.md");
        assert!(lines.len() > 0);
        assert_eq!(35, lines[0].as_bytes()[0])
    }

    #[test]
    fn test_from_file() {
        let f = from_file("README.md");
        assert!(f.len() > 0);
        assert_eq!(35, f.as_bytes()[0])
    }
}
