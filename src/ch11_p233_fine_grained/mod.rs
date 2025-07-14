use ::std::sync::Arc;
use ::tokio::sync::{Mutex, MutexGuard};
use ::tokio::time::{self, Duration};

#[allow(dead_code)]
async fn async_mutex_locker(
  mutex: Arc<Mutex<i32>>,
  id: usize,
) {
  let mut lock: MutexGuard<'_, i32> = mutex.lock().await;

  println!("before increment for task {id}");

  *lock += 1;

  time::sleep(Duration::from_millis(1)).await;

  println!("after sleep for task {id}");
}

#[cfg(test)]
mod tests {
  use std::task::Poll;

  use super::*;
  use ::tokio_test::task;

  #[tokio::test]
  async fn test_monitor_file_metadata() {
    let mutex: Arc<Mutex<i32>> = Default::default();

    let mutex_clone1: Arc<Mutex<i32>> = mutex.clone();

    let mutex_clone2: Arc<Mutex<i32>> = mutex.clone();

    let mut future1 = task::spawn(async_mutex_locker(mutex_clone1, 1));

    let mut future2 = task::spawn(async_mutex_locker(mutex_clone2, 2));

    tokio_test::assert_pending!(future1.poll());

    tokio_test::assert_pending!(future2.poll());

    for _ in 0..10 {
      tokio_test::assert_pending!(future2.poll());

      time::sleep(Duration::from_millis(1)).await;
    }

    assert_eq!(future1.poll(), Poll::Ready(()));

    time::sleep(Duration::from_millis(3)).await;

    tokio_test::assert_pending!(future2.poll());

    println!("before drop of future1");

    // TODO: Why does this unit test still pass if I remark out the drop line?
    drop(future1);

    time::sleep(Duration::from_millis(1)).await;

    assert_eq!(future2.poll(), Poll::Ready(()));

    println!("before final and third lock");

    let lock: MutexGuard<'_, i32> = mutex.lock().await;

    assert_eq!(*lock, 2);
  }
}
