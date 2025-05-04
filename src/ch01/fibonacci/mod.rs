use std::thread;

pub fn run() {
  let mut threads = Vec::new();

  for i in 0..8 {
    let handle = thread::spawn(move || {
      let result = fibonacci(40);

      println!("Thread {i} result: {result}");
    });

    threads.push(handle);
  }

  for handle in threads {
    handle.join().unwrap();
  }
}

fn fibonacci(n: u64) -> u64 {
  if n == 0 || n == 1 {
    return n;
  }

  fibonacci(n - 1) + fibonacci(n - 2)
}
