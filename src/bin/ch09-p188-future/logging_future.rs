use super::logging::Logging;

pub struct LoggingFuture<F: Future + Logging> {
  pub inner: F,
}

impl<F: Future + Logging> Future for LoggingFuture<F> {
  type Output = F::Output;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    context: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let inner = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };

    inner.log();

    inner.poll(context)
  }
}
