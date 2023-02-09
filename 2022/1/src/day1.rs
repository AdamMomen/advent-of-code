use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

pub fn solution() -> Result<i32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut running_sum: i32 = 0;
    let mut result: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let cloned_line = line?.clone();
        let is_empty = cloned_line.to_string().len() == 0;

        if is_empty {
            result.push(running_sum);
            running_sum = 0;
            continue;
        }

        let number = cloned_line.clone().parse::<i32>().unwrap();
        running_sum += number;
    }

    result.sort();

    let final_result = &result[result.len() - 3..result.len()]
        .into_iter()
        .sum();
    Ok(*final_result)
}
