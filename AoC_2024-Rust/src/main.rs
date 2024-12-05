mod day1;
use day1::day1_2;

mod day2;
use day2::*;

mod day3;
use day3::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day3_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
