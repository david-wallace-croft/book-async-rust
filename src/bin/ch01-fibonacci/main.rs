use ::book_async_rust::ch01;
use ::std::thread::{self, JoinHandle};

fn main() {
  let mut threads: Vec<JoinHandle<()>> = Vec::new();

  for i in 0..8 {
    let handle: JoinHandle<()> = thread::spawn(move || {
      let result: u64 = ch01::fibonacci(40);

      println!("Thread {i} result: {result}");
    });

    threads.push(handle);
  }

  for handle in threads {
    handle.join().unwrap();
  }
}
