#![feature(coroutine_trait)]
#![feature(coroutines)]

use self::rand_co_routine::RandCoRoutine;
use std::ops::CoroutineState;
use std::{ops::Coroutine, pin::Pin};

mod rand_co_routine;

fn main() {
  let mut coroutines: Vec<RandCoRoutine> = Default::default();

  for _ in 0..10 {
    coroutines.push(Default::default());
  }

  let mut total: u32 = 0;

  loop {
    let mut all_dead: bool = true;

    for mut coroutine in coroutines.iter_mut() {
      if coroutine.live {
        all_dead = false;

        match Pin::new(&mut coroutine).resume(()) {
          CoroutineState::Complete(_) => {
            panic!("Coroutine should not complete");
          },
          CoroutineState::Yielded(result) => {
            total += result as u32;
          },
        }

        if coroutine.value < 9 {
          coroutine.live = false;
        }
      }
    }

    if all_dead {
      break;
    }
  }

  println!("Total: {total}");
}
