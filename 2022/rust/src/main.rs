mod day1;
use std::env;
use crate::day1::Solution;
use std::io::Result;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let result = Solution.solve().unwrap();

    println!("Solution is: {:?}", result);
    Ok(())
}
