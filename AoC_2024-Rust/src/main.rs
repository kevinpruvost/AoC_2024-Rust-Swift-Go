mod day9;
use day9::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day9_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
