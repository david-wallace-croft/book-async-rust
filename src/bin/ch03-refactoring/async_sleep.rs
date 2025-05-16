#![expect(dead_code)]

use std::{
  pin::Pin,
  task::{Context, Poll},
  time::{Duration, Instant},
};

struct AsyncSleep {
  start_time: Instant,
  duration: Duration,
}

impl AsyncSleep {
  fn new(duration: Duration) -> Self {
    Self {
      start_time: Instant::now(),
      duration,
    }
  }
}

impl Future for AsyncSleep {
  type Output = bool;

  fn poll(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    let elapsed_time = self.start_time.elapsed();

    if elapsed_time >= self.duration {
      return Poll::Ready(true);
    }

    context.waker().wake_by_ref();

    Poll::Pending
  }
}
