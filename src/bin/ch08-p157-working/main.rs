use self::resp_message::RespMessage;
use ::std::sync::Arc;
use ::tokio::sync::mpsc::Sender;
use ::tokio::sync::{Mutex, MutexGuard};
use ::tokio::sync::{mpsc::Receiver, mpsc::channel, oneshot};
use ::tokio::task::JoinHandle;
use ::tokio::time::Instant;

mod resp_message;

const TASK_COUNT: usize = 10_000_000;

#[tokio::main]
async fn main() {
  using_mutex().await;

  using_actor().await;
}

async fn actor_replacement(
  state: Arc<Mutex<usize>>,
  value: usize,
) -> usize {
  let update_handle: JoinHandle<usize> = tokio::spawn(async move {
    let mut state: MutexGuard<'_, usize> = state.lock().await;

    *state += value;

    *state
  });

  update_handle.await.unwrap()
}

async fn resp_actor(mut rx: Receiver<RespMessage>) {
  let mut state: usize = 0;

  while let Some(msg) = rx.recv().await {
    state += msg.value;

    if msg.responder.send(state).is_err() {
      eprintln!("Failed to send response");
    }
  }
}

async fn using_actor() {
  let (tx, rx) = channel::<RespMessage>(TASK_COUNT);

  let _resp_actor_handle: JoinHandle<()> =
    tokio::spawn(async { resp_actor(rx).await });

  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  let now: Instant = Instant::now();

  for i in 0..TASK_COUNT {
    let tx_ref: Sender<RespMessage> = tx.clone();

    let future = async move {
      let (resp_tx, resp_rx) = oneshot::channel::<usize>();

      let msg: RespMessage = RespMessage {
        value: i,
        responder: resp_tx,
      };

      tx_ref.send(msg).await.unwrap();

      let _ = resp_rx.await.unwrap();
    };

    handles.push(tokio::spawn(future));
  }

  for handle in handles {
    let _ = handle.await.unwrap();
  }

  // Elapsed: 11.6121145s for ten million.
  println!("Elapsed: {:?}", now.elapsed());
}

async fn using_mutex() {
  let state: Arc<Mutex<usize>> = Default::default();

  let mut handles: Vec<JoinHandle<()>> = Default::default();

  let now: Instant = Instant::now();

  for i in 0..TASK_COUNT {
    let state_ref: Arc<Mutex<usize>> = state.clone();

    let future = async move {
      let handle: JoinHandle<usize> =
        tokio::spawn(async move { actor_replacement(state_ref, i).await });

      let _ = handle.await.unwrap();
    };

    let join_handle: JoinHandle<()> = tokio::spawn(future);

    handles.push(join_handle);
  }

  for handle in handles {
    let _ = handle.await.unwrap();
  }

  // Elapsed: 20.1622016s for ten million.
  // Takes disproportionately longer for one hundred million due to memory use.
  println!("Elapsed: {:?}", now.elapsed());
}
