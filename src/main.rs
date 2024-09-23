mod easy;
mod medium;
use std::time::Instant;

fn main() {
    // Record the start time
    let start = Instant::now();
    // PASTE TEST OF SOLUTION HERE
    easy::strings::longest_common_prefix::test();

    let elapsed = start.elapsed();
    // Print out the elapsed time
    println!("Elapsed time: {:?}", elapsed);
}
