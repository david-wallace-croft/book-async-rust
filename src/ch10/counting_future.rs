use ::std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

pub struct CountingFuture {
  pub count: i32,
}

impl Future for CountingFuture {
  type Output = i32;

  fn poll(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    self.count += 1;

    if self.count == 4 {
      println!("CountingFuture is done!");

      Poll::Ready(self.count)
    } else {
      cx.waker().wake_by_ref();

      println! { "CountingFuture is not done yet: {}", self.count};

      Poll::Pending
    }
  }
}
