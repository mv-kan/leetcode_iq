mod easy;
mod medium;
use std::time::Instant;

use easy::intersection_of_two_arrays_ii;

fn main() {
    // Record the start time
    let start = Instant::now();
    // PASTE TEST SOLUTION HERE
    // buy_sell_stock::test();
    intersection_of_two_arrays_ii::test();

    let elapsed = start.elapsed();
    // Print out the elapsed time
    println!("Elapsed time: {:?}", elapsed);
}
