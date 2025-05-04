use ::book_async_rust::ch01;
use ::std::time::{Duration, Instant};

fn main() {
  let start_time: Instant = Instant::now();

  let _result: u64 = ch01::fibonacci(50);

  let elapsed_time: Duration = start_time.elapsed();

  // Prints "Request took 93303 ms"

  println!("Request took {} ms", elapsed_time.as_millis());
}
