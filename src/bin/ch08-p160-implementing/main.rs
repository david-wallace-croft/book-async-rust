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

fn main() {
  todo!()
}

async fn key_value_actor(mut receiver: Receiver<KeyValueMessage>) {
  let mut map = std::collections::HashMap::new();

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
