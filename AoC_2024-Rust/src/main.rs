mod day6;
use day6::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day6_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
