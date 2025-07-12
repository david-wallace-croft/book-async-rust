#![allow(dead_code)]

use ::std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER_MULTI_THREADED: AtomicUsize = AtomicUsize::new(0);

static COUNTER_SINGLE_THREADED: AtomicUsize = AtomicUsize::new(0);

async fn unsafe_add_single_threaded() {
  let value: usize = COUNTER_SINGLE_THREADED.load(Ordering::SeqCst);

  COUNTER_SINGLE_THREADED.store(value + 1, Ordering::SeqCst);
}

async fn unsafe_add_multi_threaded() {
  let value: usize = COUNTER_MULTI_THREADED.load(Ordering::SeqCst);

  COUNTER_MULTI_THREADED.store(value + 1, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
  use super::*;
  use ::tokio::runtime::{Builder, Runtime};
  use ::tokio::task::JoinHandle;

  #[test]
  fn test_data_race_single_threaded() {
    let runtime: Runtime =
      Builder::new_current_thread().enable_all().build().unwrap();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    let total: usize = 100_000;

    for _ in 0..total {
      let handle: JoinHandle<()> = runtime.spawn(unsafe_add_single_threaded());

      handles.push(handle);
    }

    for handle in handles {
      runtime.block_on(handle).unwrap();
    }

    assert_eq!(
      COUNTER_SINGLE_THREADED.load(Ordering::SeqCst),
      total,
      "race condition occurred!"
    );
  }

  #[test]
  fn test_data_race_multi_threaded() {
    let runtime: Runtime = Runtime::new().unwrap();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    let total: usize = 100_000;

    for _ in 0..total {
      let handle: JoinHandle<()> = runtime.spawn(unsafe_add_multi_threaded());

      handles.push(handle);
    }

    for handle in handles {
      runtime.block_on(handle).unwrap();
    }

    assert_ne!(
      COUNTER_MULTI_THREADED.load(Ordering::SeqCst),
      total,
      "race condition did not occur!"
    );
  }
}
