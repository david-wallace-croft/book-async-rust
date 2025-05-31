use ::book_async_rust::ch01;
use ::std::time::{Duration, Instant};

fn main() {
  let start: Instant = Instant::now();

  let _result: u64 = ch01::fibonacci(50);

  let duration: Duration = start.elapsed();

  // Prints "fibonacci(50) in 95.5378462s"

  println!("fibonacci(50) in {duration:?}");
}
