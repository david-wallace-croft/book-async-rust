use super::future_type::FutureType;
use ::async_task::{Runnable, Task};
use ::flume::{Receiver, Sender, unbounded};
use ::std::future::Future;
use ::std::panic::catch_unwind;
use ::std::sync::LazyLock;
use ::std::thread;
use ::std::time::Duration;

#[macro_export]
macro_rules! spawn_task {
  ($future:expr) => {
    spawn_task!($future, crate::future_type::FutureType::Low)
  };

  ($future:expr, $order:expr) => {
    crate::queue::spawn_task($future, $order)
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

pub fn spawn_task<F, T>(
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

  let schedule: fn(Runnable) = match order {
    FutureType::High => schedule_high,
    FutureType::Low => schedule_low,
  };

  let (runnable, task) = async_task::spawn(future, schedule);

  runnable.schedule();

  println!("Here is the high queue count: {:?}", HIGH_QUEUE.len());

  println!("Here is the low queue count: {:?}", LOW_QUEUE.len());

  println!("Finished task spawning.");

  task
}
