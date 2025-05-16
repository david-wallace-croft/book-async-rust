use self::counter_future::CounterFuture;
use ::async_task::{Runnable, Task};
use ::futures_lite::future;
use ::std::future::Future;
use ::std::panic::catch_unwind;
use ::std::sync::LazyLock;
use ::std::thread;
use ::std::time::Duration;

mod async_sleep;
mod counter_future;

// Can we replace this with a new Rust feature?
static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
  println!("Starting queue initialization...");

  let (tx, rx) = flume::unbounded::<Runnable>();

  for _ in 0..3 {
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
  let one: CounterFuture = Default::default();

  let two: CounterFuture = Default::default();

  let t_one: Task<u32> = spawn_task(one);

  let t_two: Task<u32> = spawn_task(two);

  let t_three: Task<()> = spawn_task(async {
    async_fn().await;
    async_fn().await;
    async_fn().await;
    async_fn().await;
  });

  println!("Starting main thread sleep...");

  std::thread::sleep(Duration::from_secs(5));

  println!("Finished main thread sleep.");

  println!("Blocking on 1...");

  future::block_on(t_one);

  println!("Blocking on 2...");

  future::block_on(t_two);

  println!("Blocking on 3...");

  future::block_on(t_three);

  println!("Done");
}

async fn async_fn() {
  println!("Starting async_fn sleep...");

  std::thread::sleep(Duration::from_secs(1));

  println!("Finished async_fn sleep.");
}

fn spawn_task<F, T>(future: F) -> Task<T>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  println!("Starting task spawning...");

  let schedule = |runnable: Runnable| QUEUE.send(runnable).unwrap();

  let (runnable, task) = async_task::spawn(future, schedule);

  runnable.schedule();

  println!("Here is the queue count: {:?}", QUEUE.len());

  println!("Finished task spawning.");

  task
}
