#![expect(clippy::vec_init_then_push)]
#![expect(unused_macros)]

use self::background_process::BackgroundProcess;
use self::counter_future::CounterFuture;
use self::future_type::FutureType;
use self::runtime::Runtime;
use ::async_task::{Runnable, Task};
use ::flume::{Receiver, Sender, unbounded};
use ::futures_lite::future;
use ::std::future::Future;
use ::std::panic::catch_unwind;
use ::std::sync::LazyLock;
use ::std::thread;
use ::std::time::Duration;

mod async_sleep;
mod background_process;
mod counter_future;
mod future_order_label;
mod future_type;
mod runtime;

#[macro_export]
macro_rules! join {

  ($($future:expr),*) => {
    {
      let mut results = Vec::new();

      $(
        results.push(::futures_lite::future::block_on($future));
      )*

      results
    }
  };
}

#[macro_export]
macro_rules! spawn_task {
  ($future:expr) => {
    spawn_task!($future, FutureType::Low)
  };
  ($future:expr, $order:expr) => {
    spawn_task($future, $order)
  };
}

macro_rules! try_join {
  ($($future:expr),*) => {
    {
      let mut results = Vec::new();

      $(
        let result = catch_unwind(|| future::block_on($future));

        results.push(result);
      )*

      results
    }
  };
}

static HIGH_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
  LazyLock::new(unbounded::<Runnable>);

static LOW_CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
  LazyLock::new(unbounded::<Runnable>);

// Can we replace this with a new Rust feature?
static HIGH_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
  println!("Starting queue initialization...");

  for _ in 0..2 {
    let high_receiver: Receiver<Runnable> = HIGH_CHANNEL.1.clone();

    let low_receiver: Receiver<Runnable> = LOW_CHANNEL.1.clone();

    thread::spawn(move || {
      loop {
        match high_receiver.try_recv() {
          Ok(runnable) => {
            println!("Starting high runnable...");

            let _ = catch_unwind(|| runnable.run());

            println!("Finished high runnable.");
          },
          Err(_) => match low_receiver.try_recv() {
            Ok(runnable) => {
              println!("Starting low runnable...");

              let _ = catch_unwind(|| runnable.run());

              println!("Finished low runnable.");
            },
            Err(_) => {
              thread::sleep(Duration::from_millis(100));
            },
          },
        }
      }
    });
  }

  println!("Finished queue initialization.");

  HIGH_CHANNEL.0.clone()
});

// Can we replace this with a new Rust feature?
static LOW_QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
  println!("Starting queue initialization...");

  for _ in 0..1 {
    let high_receiver: Receiver<Runnable> = HIGH_CHANNEL.1.clone();

    let low_receiver: Receiver<Runnable> = LOW_CHANNEL.1.clone();

    thread::spawn(move || {
      loop {
        match low_receiver.try_recv() {
          Ok(runnable) => {
            println!("Starting low runnable...");

            let _ = catch_unwind(|| runnable.run());

            println!("Finished low runnable.");
          },
          Err(_) => match high_receiver.try_recv() {
            Ok(runnable) => {
              println!("Starting high runnable...");

              let _ = catch_unwind(|| runnable.run());

              println!("Finished high runnable.");
            },
            Err(_) => {
              thread::sleep(Duration::from_millis(100));
            },
          },
        }
      }
    });
  }

  println!("Finished queue initialization.");

  HIGH_CHANNEL.0.clone()
});

fn main() {
  println!("Starting Runtime...");

  Runtime::new().with_low_num(2).with_high_num(4).run();

  println!("Spawning BackgroundProcess...");

  let background_process_task: Task<()> = spawn_task!(BackgroundProcess);

  background_process_task.detach();

  let one: CounterFuture = CounterFuture::new(FutureType::High);

  let two: CounterFuture = CounterFuture::new(FutureType::Low);

  println!("Spawning Task 1...");

  let t_one: Task<u32> = spawn_task!(one, FutureType::High);

  println!("Spawning Task 2...");

  let t_two: Task<u32> = spawn_task!(two);

  println!("Spawning Task 3...");

  let t_three: Task<()> = spawn_task!(async_fn());

  println!("Spawning Task 4...");

  let t_four: Task<()> = spawn_task!(
    async {
      async_fn().await;
      async_fn().await;
    },
    FutureType::High
  );

  println!("Starting main thread sleep...");

  std::thread::sleep(Duration::from_secs(5));

  println!("Finished main thread sleep.");

  println!("Blocking on task 1...");

  future::block_on(t_one);

  println!("Blocking on task 2...");

  future::block_on(t_two);

  println!("Blocking on task 3...");

  future::block_on(t_three);

  println!("Blocking on task 4...");

  future::block_on(t_four);

  println!("Done");
}

async fn async_fn() {
  println!("Starting async_fn sleep...");

  std::thread::sleep(Duration::from_secs(1));

  println!("Finished async_fn sleep.");
}

fn spawn_task<F, T>(
  future: F,
  order: FutureType,
) -> Task<T>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  println!("Starting task spawning...");

  let schedule_high = |runnable: Runnable| HIGH_QUEUE.send(runnable).unwrap();

  let schedule_low = |runnable: Runnable| LOW_QUEUE.send(runnable).unwrap();

  let schedule = match order {
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
