use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
pub fn read_file(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("Should have been able to read the file");

    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|f| f.expect("Readline").as_str().to_string())
        .collect::<Vec<_>>();
}
