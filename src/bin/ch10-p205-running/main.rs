use ::book_async_rust::ch10::counting_future::CountingFuture;
use ::book_async_rust::ch10::executor::Executor;
use ::std::sync::mpsc::Receiver;
use ::std::thread;

fn main() {
  let counter: CountingFuture = CountingFuture {
    count: 0,
  };

  let counter_two: CountingFuture = CountingFuture {
    count: 0,
  };

  let mut executor: Executor = Default::default();

  let handle: Receiver<i32> = executor.spawn(counter);

  let _handle_two: Receiver<i32> = executor.spawn(counter_two);

  thread::spawn(move || {
    loop {
      executor.poll();
    }
  });

  let result: i32 = handle.recv().unwrap();

  println!("Result: {result}");
}
