use ::book_async_rust::ch01;
use ::std::thread;
use ::std::time::{Duration, Instant};

fn main() {
  let start: Instant = Instant::now();

  let mut handles: Vec<thread::JoinHandle<u64>> = vec![];

  for _ in 0..4 {
    let handle: thread::JoinHandle<u64> = thread::spawn(|| ch01::fibonacci(50));

    handles.push(handle);
  }

  for handle in handles {
    let _ = handle.join();
  }

  let duration: Duration = start.elapsed();

  // Prints "4 threads fibonacci(50) took 115.0048591s"

  println!("4 threads fibonacci(50) took {duration:?}");
}
