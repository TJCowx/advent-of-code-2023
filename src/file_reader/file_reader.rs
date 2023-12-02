use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_as_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("file not found");

    let mut vec = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    vec
}
