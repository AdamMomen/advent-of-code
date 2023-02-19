use std::fs::File;
use std::io::{prelude::*, BufReader, Result};
use std::path::PathBuf;

pub struct Solution;

impl Solution {

    fn read_file_lines(&self, file_path: &PathBuf) -> Result<Vec<String>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut lines = vec![];
        for line in reader.lines() {
            lines.push(line?);
        }
        Ok(lines)
    }
    pub fn solve(&self) -> Result<u32> {
        let mut running_sum: u32 = 0;
        let mut result: Vec<u32> = Vec::new();

        let mut file_path = PathBuf::new();
        file_path.push("/Users/adam/personal/advent_of_code/2022/input/day1.txt");
        print!("path: {}", file_path.to_str().unwrap());
        let lines = self.read_file_lines(&file_path).expect("Error reading the file");
        for line in lines {
            let cloned_line = line.clone();
            let is_empty = cloned_line.to_string().len() == 0;

            if is_empty {
                result.push(running_sum);
                running_sum = 0;
                continue;
            }

            let number = cloned_line.clone().parse::<u32>().unwrap();
            running_sum += number;
        }
        result.sort();

        let final_result = &result[result.len() - 3..result.len()]
            .into_iter()
            .sum();
        Ok(*final_result)
    }
}
