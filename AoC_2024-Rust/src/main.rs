mod day14;
use day14::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day14_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}