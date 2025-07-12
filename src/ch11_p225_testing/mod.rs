#[cfg(test)]
mod tests {
  use ::std::sync::Arc;
  use ::tokio::sync::{Mutex, MutexGuard};
  use ::tokio::task::JoinHandle;
  use ::tokio::time::error::Elapsed;
  use ::tokio::time::{Duration, sleep, timeout};

  #[tokio::test]
  async fn test_deadlock_detection() {
    let resource1: Arc<Mutex<i32>> = Default::default();

    let resource2: Arc<Mutex<i32>> = Default::default();

    let resource1_clone: Arc<Mutex<i32>> = Arc::clone(&resource1);

    let resource2_clone: Arc<Mutex<i32>> = Arc::clone(&resource2);

    let handle1: JoinHandle<()> = tokio::spawn(async move {
      let _lock1: MutexGuard<'_, i32> = resource1.lock().await;

      sleep(Duration::from_millis(100)).await;

      let _lock2: MutexGuard<'_, i32> = resource2.lock().await;
    });

    let handle2: JoinHandle<()> = tokio::spawn(async move {
      let _lock2: MutexGuard<'_, i32> = resource2_clone.lock().await;

      sleep(Duration::from_millis(100)).await;

      let _lock1: MutexGuard<'_, i32> = resource1_clone.lock().await;
    });

    let result: Result<(), Elapsed> = timeout(Duration::from_secs(5), async {
      let _ = handle1.await;

      let _ = handle2.await;
    })
    .await;

    assert!(result.is_err(), "Deadlock not detected!");
  }
}
