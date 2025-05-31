use ::std::{
  ops::{Coroutine, CoroutineState},
  pin::Pin,
  task::{Context, Poll},
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

impl Future for SleepCoroutine {
  type Output = ();

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    match Pin::new(&mut self).resume(()) {
      CoroutineState::Complete(_) => Poll::Ready(()),
      CoroutineState::Yielded(_) => {
        context.waker().wake_by_ref();

        Poll::Pending
      },
    }
  }
}
