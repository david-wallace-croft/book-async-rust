use ::std::sync::Arc;
use ::tokio::sync::{Mutex, MutexGuard};
use ::tokio::task::JoinHandle;
use ::tokio::time::Instant;

#[tokio::main]
async fn main() {
  let state: Arc<Mutex<i64>> = Default::default();

  let mut handles: Vec<JoinHandle<()>> = Default::default();

  let now: Instant = Instant::now();

  for i in 0..10_000_000 {
    let state_ref: Arc<Mutex<i64>> = state.clone();

    let future = async move {
      let handle: JoinHandle<i64> =
        tokio::spawn(async move { actor_replacement(state_ref, i).await });

      let _ = handle.await.unwrap();
    };

    let join_handle: JoinHandle<()> = tokio::spawn(future);

    handles.push(join_handle);
  }

  for handle in handles {
    let _ = handle.await.unwrap();
  }

  // Elapsed: 18.6762593s for ten million.
  // Takes disproportionately longer for one hundred million due to memory use.
  println!("Elapsed: {:?}", now.elapsed());
}

async fn actor_replacement(
  state: Arc<Mutex<i64>>,
  value: i64,
) -> i64 {
  let mut state: MutexGuard<'_, i64> = state.lock().await;

  *state += value;

  *state
}
