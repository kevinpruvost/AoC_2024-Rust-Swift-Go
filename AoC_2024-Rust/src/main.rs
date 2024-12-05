mod day5;
use day5::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day5_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
