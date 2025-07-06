use ::std::future::Future;
use ::std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use ::tokio::task::{self, JoinHandle};

static COUNT: AtomicUsize = AtomicUsize::new(0);
static OPEN: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> Result<(), String> {
  let _ = spawn_task(passing_task())?.await;

  let _ = spawn_task(error_task())?.await;

  let _ = spawn_task(error_task())?.await;

  let _ = spawn_task(error_task())?.await;

  let _ = spawn_task(passing_task())?.await;

  Ok(())
}

async fn error_task() {
  println!("error task running");

  let count: usize = COUNT.fetch_add(1, Ordering::SeqCst);

  if count == 2 {
    println!("opening circuit");

    OPEN.store(true, Ordering::SeqCst);
  }
}

async fn passing_task() {
  println!("passing task running");
}

fn spawn_task<F, T>(future: F) -> Result<JoinHandle<T>, String>
where
  F: Future<Output = T> + Send + 'static,
  T: Send + 'static,
{
  let open: bool = OPEN.load(Ordering::SeqCst);

  if !open {
    return Ok(task::spawn(future));
  }

  Err("Circuit Open".to_string())
}
