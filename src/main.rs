mod easy;
mod medium;
use std::time::Instant;

use easy::{ rotate_array};

fn main() {
    // Record the start time
    let start = Instant::now();
    // PASTE TEST SOLUTION HERE
    // buy_sell_stock::test();
    rotate_array::test();

    let elapsed = start.elapsed();
    // Print out the elapsed time
    println!("Elapsed time: {:?}", elapsed);
}
