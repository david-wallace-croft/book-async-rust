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
  let (tx, rx) = flume::unbounded::<Runnable>();

  thread::spawn(move || {
    while let Ok(runnable) = rx.recv() {
      println!("runnable accepted");

      let _ = catch_unwind(|| runnable.run());
    }
  });

  tx
});

fn main() {
  let one: CounterFuture = Default::default();

  let two: CounterFuture = Default::default();

  let t_one: Task<u32> = spawn_task(one);

  let t_two: Task<u32> = spawn_task(two);

  let t_three = spawn_task(async {
    async_fn().await;
    async_fn().await;
    async_fn().await;
    async_fn().await;
  });

  std::thread::sleep(Duration::from_secs(5));

  println!("Before the block");

  future::block_on(t_one);

  future::block_on(t_two);

  future::block_on(t_three);
}

async fn async_fn() {
  std::thread::sleep(Duration::from_secs(1));

  println!("async fn");
}

fn spawn_task<F, T>(future: F) -> Task<T>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  let schedule = |runnable: Runnable| QUEUE.send(runnable).unwrap();

  let (runnable, task) = async_task::spawn(future, schedule);

  runnable.schedule();

  println!("Here is the queue count: {:?}", QUEUE.len());

  task
}
