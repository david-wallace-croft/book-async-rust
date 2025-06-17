use self::message::Message;
use tokio::sync::{
  mpsc::channel,
  mpsc::{Receiver, Sender},
  oneshot,
};
use tokio::task::JoinHandle;

mod message;

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
}

async fn basic_actor(mut rx: Receiver<Message>) {
  let mut state: i64 = 0;

  while let Some(msg) = rx.recv().await {
    state += msg.value;

    println!("Received: {}", msg.value);

    println!("State: {state}");
  }
}
