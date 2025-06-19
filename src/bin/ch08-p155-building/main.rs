use self::message::Message;
use self::resp_message::RespMessage;
use tokio::sync::{mpsc::Receiver, mpsc::channel, oneshot};
use tokio::task::JoinHandle;

mod message;
mod resp_message;

#[tokio::main]
async fn main() {
  let (tx, rx) = channel::<Message>(100);

  let _actor_handle: JoinHandle<()> = tokio::spawn(basic_actor(rx));

  for i in 0..10 {
    let msg: Message = Message {
      value: i,
    };

    tx.send(msg).await.unwrap();
  }

  send_to_resp_actor().await
}

async fn basic_actor(mut rx: Receiver<Message>) {
  let mut state: i64 = 0;

  while let Some(msg) = rx.recv().await {
    state += msg.value;

    println!("Received: {}", msg.value);

    println!("State: {state}");
  }
}

async fn resp_actor(mut rx: Receiver<RespMessage>) {
  let mut state: i64 = 0;

  while let Some(msg) = rx.recv().await {
    state += msg.value;

    if msg.responder.send(state).is_err() {
      eprintln!("Failed to send response");
    }
  }
}

async fn send_to_resp_actor() {
  let (tx, rx) = channel::<RespMessage>(100);

  let _resp_actor_handle: JoinHandle<()> = tokio::spawn(async {
    resp_actor(rx).await;
  });

  for i in 0..10 {
    let (resp_tx, resp_rx) = oneshot::channel::<i64>();

    let msg: RespMessage = RespMessage {
      value: i,
      responder: resp_tx,
    };

    tx.send(msg).await.unwrap();

    println!("Response: {}", resp_rx.await.unwrap());
  }
}
