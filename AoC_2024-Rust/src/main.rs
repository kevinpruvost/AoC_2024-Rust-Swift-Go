mod day11;
use day11::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day11_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
