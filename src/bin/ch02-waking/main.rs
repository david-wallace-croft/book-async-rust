use ::std::pin::Pin;
use ::std::sync::{Arc, Mutex, MutexGuard};
use ::std::task::{Context, Poll, Waker};
use ::tokio::sync::mpsc;
use ::tokio::task::{self, JoinHandle};

#[derive(Default)]
struct MyFutureState {
  data: Option<Vec<u8>>,
  waker: Option<Waker>,
}

struct MyFuture {
  state: Arc<Mutex<MyFutureState>>,
}

impl MyFuture {
  fn new() -> (Self, Arc<Mutex<MyFutureState>>) {
    let state: Arc<Mutex<MyFutureState>> = Default::default();

    (
      MyFuture {
        state: state.clone(),
      },
      state,
    )
  }
}

impl Future for MyFuture {
  type Output = String;

  fn poll(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    println!("Polling the future");

    let mut state: MutexGuard<'_, MyFutureState> = self.state.lock().unwrap();

    if state.data.is_some() {
      let data: Vec<u8> = state.data.take().unwrap();

      return Poll::Ready(String::from_utf8(data).unwrap());
    }

    state.waker = Some(context.waker().clone());

    Poll::Pending
  }
}

#[tokio::main]
async fn main() {
  let (my_future, state) = MyFuture::new();

  let (tx, mut rx) = mpsc::channel::<()>(1);

  let task_handle: JoinHandle<String> = task::spawn(my_future);

  tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

  println!("spawning trigger task");

  let trigger_task: JoinHandle<()> = task::spawn(async move {
    rx.recv().await;

    let mut state: MutexGuard<'_, MyFutureState> = state.lock().unwrap();

    state.data = Some(b"Hello from the outside".to_vec());

    loop {
      if let Some(waker) = state.waker.take() {
        waker.wake();

        break;
      }
    }
  });

  tx.send(()).await.unwrap();

  let outcome: String = task_handle.await.unwrap();

  println!("Task completed with outcome: {}", outcome);

  trigger_task.await.unwrap();
}
