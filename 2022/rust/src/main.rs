mod day2;
use std::env;
use crate::day2::Solution;
use std::io::Result;

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let result = Solution::new().solve();

    println!("Solution is: {:?}", result);
    Ok(())
}
