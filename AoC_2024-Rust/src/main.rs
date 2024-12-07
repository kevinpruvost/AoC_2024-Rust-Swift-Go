mod day7;
use day7::*;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    day7_1();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    Ok(())
}
