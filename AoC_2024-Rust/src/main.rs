mod day8;
use day8::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day8_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
