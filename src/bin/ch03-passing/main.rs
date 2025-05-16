use self::counter_future::CounterFuture;
use self::future_order_label::FutureOrderLabel;
use self::future_type::FutureType;
use ::async_task::{Runnable, Task};
use ::futures_lite::future;
use ::std::future::Future;
use ::std::panic::catch_unwind;
use ::std::sync::LazyLock;
use ::std::thread;
use ::std::time::Duration;

mod async_sleep;
mod counter_future;
mod future_order_label;
mod future_type;

// Can we replace this with a new Rust feature?
static HIGH_QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
  println!("Starting queue initialization...");

  let (tx, rx) = flume::unbounded::<Runnable>();

  for _ in 0..2 {
    let receiver = rx.clone();

    thread::spawn(move || {
      while let Ok(runnable) = receiver.recv() {
        println!("Starting runnable...");

        let _ = catch_unwind(|| runnable.run());

        println!("Finished runnable.");
      }
    });
  }

  println!("Finished queue initialization.");

  tx
});

// Can we replace this with a new Rust feature?
static LOW_QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
  println!("Starting queue initialization...");

  let (tx, rx) = flume::unbounded::<Runnable>();

  for _ in 0..1 {
    let receiver = rx.clone();

    thread::spawn(move || {
      while let Ok(runnable) = receiver.recv() {
        println!("Starting runnable...");

        let _ = catch_unwind(|| runnable.run());

        println!("Finished runnable.");
      }
    });
  }

  println!("Finished queue initialization.");

  tx
});

fn main() {
  let one: CounterFuture = CounterFuture::new(FutureType::High);

  let two: CounterFuture = CounterFuture::new(FutureType::Low);

  let t_one: Task<u32> = spawn_task(one);

  let t_two: Task<u32> = spawn_task(two);

  // let t_three: Task<()> = spawn_task(async {
  //   async_fn().await;
  //   async_fn().await;
  //   async_fn().await;
  //   async_fn().await;
  // });

  println!("Starting main thread sleep...");

  std::thread::sleep(Duration::from_secs(5));

  println!("Finished main thread sleep.");

  println!("Blocking on 1...");

  future::block_on(t_one);

  println!("Blocking on 2...");

  future::block_on(t_two);

  println!("Blocking on 3...");

  // future::block_on(t_three);

  println!("Done");
}

// async fn async_fn() {
//   println!("Starting async_fn sleep...");

//   std::thread::sleep(Duration::from_secs(1));

//   println!("Finished async_fn sleep.");
// }

fn spawn_task<F, T>(future: F) -> Task<T>
where
  F: Future<Output = T> + Send + 'static + FutureOrderLabel,
  T: Send + 'static,
{
  println!("Starting task spawning...");

  let schedule_high = |runnable: Runnable| HIGH_QUEUE.send(runnable).unwrap();

  let schedule_low = |runnable: Runnable| LOW_QUEUE.send(runnable).unwrap();

  let schedule = match future.get_order() {
    future_type::FutureType::High => schedule_high,
    future_type::FutureType::Low => schedule_low,
  };

  let (runnable, task) = async_task::spawn(future, schedule);

  runnable.schedule();

  println!("Here is the high queue count: {:?}", HIGH_QUEUE.len());

  println!("Here is the low queue count: {:?}", LOW_QUEUE.len());

  println!("Finished task spawning.");

  task
}
