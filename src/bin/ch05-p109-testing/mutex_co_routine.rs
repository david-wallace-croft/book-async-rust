use ::std::{
  ops::{Coroutine, CoroutineState},
  pin::Pin,
  sync::{Arc, Mutex},
};

#[expect(dead_code)]
pub struct MutexCoRoutine {
  pub handle: Arc<Mutex<u8>>,
  pub threshold: u8,
}

impl Coroutine<()> for MutexCoRoutine {
  type Yield = ();

  type Return = ();

  fn resume(
    mut self: Pin<&mut Self>,
    _arg: (),
  ) -> CoroutineState<Self::Yield, Self::Return> {
    match self.handle.try_lock() {
      Ok(mut handle) => {
        *handle += 1;
      },
      Err(_) => {
        return CoroutineState::Yielded(());
      },
    }

    // TODO: Is this saturationg subtraction supposed to be necessary?
    self.threshold = self.threshold.saturating_sub(1);

    if self.threshold == 0 {
      return CoroutineState::Complete(());
    }

    CoroutineState::Yielded(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ::tokio::task::JoinHandle;
  use std::{
    future::Future,
    sync::MutexGuard,
    task::{Context, Poll},
  };

  fn check_yield(coroutine: &mut MutexCoRoutine) -> bool {
    match Pin::new(coroutine).resume(()) {
      CoroutineState::Complete(_) => false,
      CoroutineState::Yielded(_) => true,
    }
  }

  impl Future for MutexCoRoutine {
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

  #[test]
  fn basic_test() {
    let handle: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

    let mut first_coroutine: MutexCoRoutine = MutexCoRoutine {
      handle: handle.clone(),
      threshold: 2,
    };

    let mut second_coroutine: MutexCoRoutine = MutexCoRoutine {
      handle: handle.clone(),
      threshold: 2,
    };

    let lock: MutexGuard<'_, u8> = handle.lock().unwrap();

    for _ in 0..3 {
      assert_eq!(check_yield(&mut first_coroutine), true);
      assert_eq!(check_yield(&mut second_coroutine), true);
    }

    assert_eq!(*lock, 0);

    std::mem::drop(lock);

    assert_eq!(check_yield(&mut first_coroutine), true);

    assert_eq!(*handle.lock().unwrap(), 1);

    assert_eq!(check_yield(&mut second_coroutine), true);

    assert_eq!(*handle.lock().unwrap(), 2);

    assert_eq!(check_yield(&mut first_coroutine), false);

    assert_eq!(*handle.lock().unwrap(), 3);

    assert_eq!(check_yield(&mut first_coroutine), false);

    assert_eq!(*handle.lock().unwrap(), 4);
  }

  #[tokio::test]
  async fn async_test() {
    let handle: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));

    let first_coroutine: MutexCoRoutine = MutexCoRoutine {
      handle: handle.clone(),
      threshold: 2,
    };

    let second_coroutine: MutexCoRoutine = MutexCoRoutine {
      handle: handle.clone(),
      threshold: 2,
    };

    let handle_one: JoinHandle<()> =
      tokio::spawn(async move { first_coroutine.await });

    let handle_two: JoinHandle<()> =
      tokio::spawn(async move { second_coroutine.await });

    handle_one.await.unwrap();

    handle_two.await.unwrap();

    assert_eq!(*handle.lock().unwrap(), 4);
  }
}
