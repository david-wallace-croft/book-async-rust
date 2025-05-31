use ::std::{
  ops::{Coroutine, CoroutineState},
  pin::Pin,
};
use rand::prelude::*;

pub struct RandCoRoutine {
  pub live: bool,
  pub value: u8,
}

impl RandCoRoutine {
  pub fn generate(&mut self) {
    let mut rng: ThreadRng = rand::rng();

    self.value = rng.random_range(0..=10);
  }
}

impl Coroutine<()> for RandCoRoutine {
  type Yield = u8;

  type Return = ();

  fn resume(
    mut self: Pin<&mut Self>,
    _arg: (),
  ) -> CoroutineState<Self::Yield, Self::Return> {
    self.generate();

    CoroutineState::Yielded(self.value)
  }
}

impl Default for RandCoRoutine {
  fn default() -> Self {
    let mut coroutine: RandCoRoutine = Self {
      live: true,
      value: 0,
    };

    coroutine.generate();

    coroutine
  }
}
