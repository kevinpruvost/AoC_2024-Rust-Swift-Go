mod day10;
use day10::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day10_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
