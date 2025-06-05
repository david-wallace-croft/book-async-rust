use super::statics::TEMP;
use ::core::sync::atomic::Ordering;
use ::std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
  time::{Duration, Instant},
};

pub struct HeatLossFuture {
  pub time_snapshot: Instant,
}

impl Default for HeatLossFuture {
  fn default() -> Self {
    Self {
      time_snapshot: Instant::now(),
    }
  }
}

impl Future for HeatLossFuture {
  type Output = ();

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    let current_snapshot: Instant = Instant::now();

    if current_snapshot.duration_since(self.time_snapshot)
      > Duration::from_secs(3)
    {
      TEMP.fetch_sub(1, Ordering::SeqCst);

      self.time_snapshot = Instant::now();
    }

    context.waker().wake_by_ref();

    Poll::Pending
  }
}
