#![allow(dead_code)]

use ::std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn unsafe_add() {
  let value: usize = COUNTER.load(Ordering::SeqCst);

  COUNTER.store(value + 1, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
  use super::*;
  use ::tokio::runtime::{Builder, Runtime};
  use ::tokio::task::JoinHandle;

  #[test]
  fn test_data_race() {
    let runtime: Runtime =
      Builder::new_current_thread().enable_all().build().unwrap();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    let total: usize = 100_000;

    for _ in 0..total {
      let handle: JoinHandle<()> = runtime.spawn(unsafe_add());

      handles.push(handle);
    }

    for handle in handles {
      runtime.block_on(handle).unwrap();
    }

    assert_eq!(
      COUNTER.load(Ordering::SeqCst),
      total,
      "race condition occurred!"
    );
  }
}
