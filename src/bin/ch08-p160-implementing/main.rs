use ::std::collections::HashMap;
use ::std::sync::OnceLock;
use ::tokio::sync::{
  mpsc::channel,
  mpsc::{Receiver, Sender},
  oneshot,
};

static ROUTER_SENDER: OnceLock<Sender<RoutingMessage>> = OnceLock::new();

struct SetKeyValueMessage {
  key: String,
  value: Vec<u8>,
  response: oneshot::Sender<()>,
}

struct GetKeyValueMessage {
  key: String,
  response: oneshot::Sender<Option<Vec<u8>>>,
}

struct DeleteKeyValueMessage {
  key: String,
  response: oneshot::Sender<()>,
}

enum KeyValueMessage {
  Get(GetKeyValueMessage),
  Delete(DeleteKeyValueMessage),
  Set(SetKeyValueMessage),
}

enum RoutingMessage {
  KeyValue(KeyValueMessage),
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let (sender, receiver) = channel(32);

  ROUTER_SENDER.set(sender).unwrap();

  tokio::spawn(router(receiver));

  set("hello".to_owned(), b"world".to_vec()).await?;

  let value: Option<Vec<u8>> = get("hello".to_owned()).await?;

  println!("value: {:?}", String::from_utf8(value.unwrap()));

  let _ = delete("hello".to_owned()).await;

  let value: Option<Vec<u8>> = get("hello".to_owned()).await?;

  println!("value: {value:?}");

  Ok(())
}

async fn delete(key: String) -> Result<(), std::io::Error> {
  let (tx, rx) = oneshot::channel();

  let key_value_message: KeyValueMessage =
    KeyValueMessage::Delete(DeleteKeyValueMessage {
      key,
      response: tx,
    });

  let routing_message: RoutingMessage =
    RoutingMessage::KeyValue(key_value_message);

  ROUTER_SENDER
    .get()
    .unwrap()
    .send(routing_message)
    .await
    .unwrap();

  rx.await.unwrap();

  Ok(())
}

async fn get(key: String) -> Result<Option<Vec<u8>>, std::io::Error> {
  let (tx, rx) = oneshot::channel();

  let key_value_message: KeyValueMessage =
    KeyValueMessage::Get(GetKeyValueMessage {
      key,
      response: tx,
    });

  let routing_message: RoutingMessage =
    RoutingMessage::KeyValue(key_value_message);

  ROUTER_SENDER
    .get()
    .unwrap()
    .send(routing_message)
    .await
    .unwrap();

  Ok(rx.await.unwrap())
}

async fn key_value_actor(mut receiver: Receiver<KeyValueMessage>) {
  let mut map: HashMap<String, Vec<u8>> = HashMap::new();

  while let Some(message) = receiver.recv().await {
    match message {
      KeyValueMessage::Get(get_key_value_message) => {
        let GetKeyValueMessage {
          key,
          response,
        } = get_key_value_message;

        let _ = response.send(map.get(&key).cloned());
      },
      KeyValueMessage::Delete(delete_key_value_message) => {
        let DeleteKeyValueMessage {
          key,
          response,
        } = delete_key_value_message;

        map.remove(&key);

        let _ = response.send(());
      },
      KeyValueMessage::Set(set_key_value_message) => {
        let SetKeyValueMessage {
          key,
          response,
          value,
        } = set_key_value_message;

        map.insert(key, value);

        let _ = response.send(());
      },
    }
  }
}

async fn router(mut receiver: Receiver<RoutingMessage>) {
  let (key_value_sender, key_value_receiver) = channel(32);

  tokio::spawn(key_value_actor(key_value_receiver));

  while let Some(message) = receiver.recv().await {
    match message {
      RoutingMessage::KeyValue(message) => {
        let _ = key_value_sender.send(message).await;
      },
    }
  }
}

async fn set(
  key: String,
  value: Vec<u8>,
) -> Result<(), std::io::Error> {
  let (tx, rx) = oneshot::channel();

  let key_value_message: KeyValueMessage =
    KeyValueMessage::Set(SetKeyValueMessage {
      key,
      value,
      response: tx,
    });

  let routing_message: RoutingMessage =
    RoutingMessage::KeyValue(key_value_message);

  ROUTER_SENDER
    .get()
    .unwrap()
    .send(routing_message)
    .await
    .unwrap();

  rx.await.unwrap();

  Ok(())
}
