use std::time::{Duration, Instant};

// Benchmark how fast your computer can increment a counter
fn main() {
    let mut count = 0;

    //Creates a Duration that represents 1 second
    let time_limit = Duration::new(1, 0);

    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}
