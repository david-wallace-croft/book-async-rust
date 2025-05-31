use ::std::{
  ops::{Coroutine, CoroutineState},
  pin::Pin,
  time::{Duration, Instant},
};

pub struct SleepCoroutine {
  pub duration: Duration,
  pub start: Instant,
}

impl SleepCoroutine {
  pub fn new(duration: Duration) -> Self {
    Self {
      duration,
      start: Instant::now(),
    }
  }
}

impl Coroutine<()> for SleepCoroutine {
  type Yield = ();

  type Return = ();

  fn resume(
    self: Pin<&mut Self>,
    _arg: (),
  ) -> CoroutineState<Self::Yield, Self::Return> {
    if self.start.elapsed() >= self.duration {
      CoroutineState::Complete(())
    } else {
      CoroutineState::Yielded(())
    }
  }
}
