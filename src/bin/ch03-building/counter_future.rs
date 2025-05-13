use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};

#[derive(Default)]
pub struct CounterFuture {
  pub count: u32,
}

impl Future for CounterFuture {
  type Output = u32;

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    self.count += 1;

    println!("polling with result: {}", self.count);

    std::thread::sleep(Duration::from_secs(1));

    if self.count < 3 {
      context.waker().wake_by_ref();
      return Poll::Pending;
    }

    Poll::Ready(self.count)
  }
}
