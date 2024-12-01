mod day1;

use std::time::Instant;

fn main() {
    println!("Hello, world!");
    
    // Benchmark the `day1::main::main()` function
    let start = Instant::now();

    day1::main::main();

    let duration = start.elapsed();
    println!("Execution time of day1::main::main: {:?}", duration);
}
