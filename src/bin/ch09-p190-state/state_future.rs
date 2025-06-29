use super::state::State;
use ::std::pin::Pin;
use ::std::task::Context;
use ::std::task::Poll;

#[expect(dead_code)]
pub struct StateFuture<F: Future, X: Future> {
  pub off_future: X,
  pub on_future: F,
  pub state: State,
}

impl<F: Future, X: Future> Future for StateFuture<F, X> {
  type Output = State;

  fn poll(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    match self.state {
      State::Off => {
        let inner: Pin<&mut F> = unsafe {
          self.map_unchecked_mut(|s: &mut StateFuture<F, X>| &mut s.on_future)
        };

        let _ = inner.poll(cx);

        cx.waker().wake_by_ref();

        Poll::Pending
      },
      State::On => {
        let inner: Pin<&mut X> = unsafe {
          self.map_unchecked_mut(|s: &mut StateFuture<F, X>| &mut s.off_future)
        };

        let _ = inner.poll(cx);

        cx.waker().wake_by_ref();

        Poll::Pending
      },
    }
  }
}
