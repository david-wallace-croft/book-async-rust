use std::{
  pin::Pin,
  task::{Context, Poll},
};

pub struct NestingFuture {
  pub inner: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Future for NestingFuture {
  type Output = ();

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    match self.inner.as_mut().poll(context) {
      Poll::Pending => Poll::Pending,
      Poll::Ready(_) => Poll::Ready(()),
    }
  }
}
