#![feature(coroutine_trait)]
#![feature(coroutines)]

use self::executor::Executor;
use self::sleep_coroutine::SleepCoroutine;
use ::std::time::{Duration, Instant};

mod executor;
mod sleep_coroutine;

fn main() {
  let mut executor: Executor = Default::default();

  for _ in 0..3 {
    let coroutine: SleepCoroutine = SleepCoroutine::new(Duration::from_secs(1));

    executor.add(Box::pin(coroutine));
  }

  let start: Instant = Instant::now();

  while !executor.coroutines.is_empty() {
    executor.poll();
  }

  println!("Took {:?}", start.elapsed());
}
