use std::env;
mod day1;
use std::io::{Result};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let result = day1::solution()?;

    println!("Solution is: {:?}", result);
    Ok(())
}
