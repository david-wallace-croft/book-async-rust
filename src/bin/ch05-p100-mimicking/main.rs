#![feature(coroutines)]
#![feature(coroutine_trait)]

use self::sleep_coroutine::SleepCoroutine;
use ::std::{
  collections::VecDeque,
  ops::{Coroutine, CoroutineState},
  pin::Pin,
  time::{Duration, Instant},
};

mod sleep_coroutine;

fn main() {
  let mut sleep_coroutines: VecDeque<SleepCoroutine> = VecDeque::new();

  sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));

  sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));

  sleep_coroutines.push_back(SleepCoroutine::new(Duration::from_secs(1)));

  let mut counter = 0;

  let start: Instant = Instant::now();

  while counter < sleep_coroutines.len() {
    let mut coroutine: SleepCoroutine = sleep_coroutines.pop_front().unwrap();

    match Pin::new(&mut coroutine).resume(()) {
      CoroutineState::Yielded(_) => {
        sleep_coroutines.push_back(coroutine);
      },
      CoroutineState::Complete(_) => {
        counter += 1;
      },
    }
  }

  println!("Took {:?}", start.elapsed());
}
