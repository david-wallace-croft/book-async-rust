use ::std::sync::LazyLock;
use ::std::thread;
use ::std::time::Duration;
use ::tokio::runtime::Builder;
use ::tokio::runtime::Runtime;
use ::tokio::task::JoinHandle;
use ::tokio::time;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
  Builder::new_multi_thread()
    .worker_threads(4)
    .max_blocking_threads(1)
    .on_thread_start(|| {
      println!("thread starting for runtime A");
    })
    .on_thread_stop(|| {
      println!("thread stopping for runtime A");
    })
    .thread_keep_alive(Duration::from_secs(60))
    .global_queue_interval(61)
    .on_thread_park(|| {
      println!("thread parking for runtime A");
    })
    .thread_name("our custom runtime A")
    .thread_stack_size(3 * 1_024 * 1_024)
    .enable_time()
    .build()
    .unwrap()
});

fn main() {
  let handle: JoinHandle<i32> = spawn_task(sleep_example());

  println!("spawned task");

  println!("task status: {}", handle.is_finished());

  thread::sleep(Duration::from_secs(3));

  println!("task status: {}", handle.is_finished());

  let result: i32 = RUNTIME.block_on(handle).unwrap();

  println!("task result: {result}");
}

async fn sleep_example() -> i32 {
  println!("sleeping for 2 seconds");

  time::sleep(Duration::from_secs(2)).await;

  println!("done sleeping");

  20
}

fn spawn_task<F, T>(future: F) -> JoinHandle<T>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  RUNTIME.spawn(future)
}
