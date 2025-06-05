use super::statics::{HEAT_ON, TEMP};
use ::core::sync::atomic::Ordering;
use ::std::{
  future::Future,
  pin::Pin,
  sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, AtomicI16},
  },
  task::{Context, Poll},
  time::{Duration, Instant},
};

pub struct HeaterFuture {
  pub time_snapshot: Instant,
}

impl Default for HeaterFuture {
  fn default() -> Self {
    Self {
      time_snapshot: Instant::now(),
    }
  }
}

impl Future for HeaterFuture {
  type Output = ();

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    if !HEAT_ON.load(Ordering::SeqCst) {
      self.time_snapshot = Instant::now();

      context.waker().wake_by_ref();

      return Poll::Pending;
    }

    let current_snapshot: Instant = Instant::now();

    if current_snapshot.duration_since(self.time_snapshot)
      < Duration::from_secs(3)
    {
      context.waker().wake_by_ref();

      return Poll::Pending;
    }

    TEMP.fetch_add(3, Ordering::SeqCst);

    self.time_snapshot = Instant::now();

    context.waker().wake_by_ref();

    return Poll::Pending;
  }
}
