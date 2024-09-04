mod easy;
mod medium;
use std::time::Instant;

use easy::signle_number;

fn main() {
    // Record the start time
    let start = Instant::now();
    // PASTE TEST SOLUTION HERE
    // buy_sell_stock::test();
    signle_number::test();

    let elapsed = start.elapsed();
    // Print out the elapsed time
    println!("Elapsed time: {:?}", elapsed);
}
