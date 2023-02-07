use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error, Lines, Result};

fn main() {
    let mut result = [];
    let lines = get_file_lines("./input.txt");
    let mut running_sum = 0;
    for line in lines {
        println!("line: {}", line);
    }
}

fn get_file_lines(filename: &str) -> ?Lines<BufReader<File>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    reader.lines()
}
