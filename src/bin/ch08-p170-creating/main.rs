#![expect(dead_code)]

use self::actor_type::ActorType;
use self::delete_key_value_message::DeleteKeyValueMessage;
use self::get_key_value_message::GetKeyValueMessage;
use self::key_value_message::KeyValueMessage;
use self::routing_message::RoutingMessage;
use self::set_key_value_message::SetKeyValueMessage;
use self::writer_log_message::WriterLogMessage;
use ::serde_json;
use ::std::collections::HashMap;
use ::std::sync::OnceLock;
use ::tokio::fs::File;
use ::tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use ::tokio::sync::{
  mpsc::channel,
  mpsc::{Receiver, Sender},
  oneshot,
};
use ::tokio::task::JoinHandle;
use ::tokio::time::{self, Duration, Instant};

mod actor_type;
mod delete_key_value_message;
mod get_key_value_message;
mod key_value_message;
mod routing_message;
mod set_key_value_message;
mod writer_log_message;

static ROUTER_SENDER: OnceLock<Sender<RoutingMessage>> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let (sender, receiver) = channel(32);

  ROUTER_SENDER.set(sender).unwrap();

  tokio::spawn(router(receiver));

  set("hello".to_owned(), b"world".to_vec()).await?;

  let value: Option<Vec<u8>> = get("hello".to_owned()).await?;

  println!("value: {value:?}");

  let value: Option<Vec<u8>> = get("hello".to_owned()).await?;

  println!("value: {value:?}");

  ROUTER_SENDER
    .get()
    .unwrap()
    .send(RoutingMessage::Reset(ActorType::KeyValue))
    .await
    .unwrap();

  let value: Option<Vec<u8>> = get("hello".to_owned()).await?;

  println!("value: {value:?}");

  set("test".to_owned(), b"world".to_vec()).await?;

  std::thread::sleep(std::time::Duration::from_secs(1));

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

async fn heartbeat_actor(mut receiver: Receiver<ActorType>) {
  let mut map = HashMap::new();

  let timeout_duration = Duration::from_millis(200);

  loop {
    match time::timeout(timeout_duration, receiver.recv()).await {
      Ok(Some(actor_name)) => {
        map.insert(actor_name, Instant::now());
      },
      Ok(None) => break,
      Err(_) => {
        continue;
      },
    }

    let half_second_ago = Instant::now() - Duration::from_millis(500);

    for (key, &value) in map.iter() {
      if value < half_second_ago {
        match key {
          ActorType::KeyValue | ActorType::Writer => {
            ROUTER_SENDER
              .get()
              .unwrap()
              .send(RoutingMessage::Reset(ActorType::KeyValue))
              .await
              .unwrap();

            map.remove(&ActorType::KeyValue);

            map.remove(&ActorType::Writer);

            break;
          },
        }
      }
    }
  }
}

async fn key_value_actor(mut receiver: Receiver<KeyValueMessage>) {
  let (writer_key_value_sender, writer_key_value_receiver) = channel(32);

  let _writer_handle: JoinHandle<Result<(), std::io::Error>> =
    tokio::spawn(writer_actor(writer_key_value_receiver));

  let (get_sender, get_receiver) = oneshot::channel();

  let _ = writer_key_value_sender
    .send(WriterLogMessage::Get(get_sender))
    .await;

  let mut map: HashMap<String, Vec<u8>> = get_receiver.await.unwrap();

  let timeout_duration = Duration::from_millis(200);

  let router_sender = ROUTER_SENDER.get().unwrap().clone();

  loop {
    match time::timeout(timeout_duration, receiver.recv()).await {
      Ok(Some(message)) => {
        if let Some(write_message) =
          WriterLogMessage::from_key_value_message(&message)
        {
          let _ = writer_key_value_sender.send(write_message).await;
        }

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
      },
      Ok(None) => break,
      Err(_) => {
        router_sender
          .send(RoutingMessage::Heartbeat(ActorType::KeyValue))
          .await
          .unwrap();
      },
    }
  }
}

async fn load_map(file_path: &str) -> HashMap<String, Vec<u8>> {
  match read_data_from_file(file_path).await {
    Ok(data) => {
      println!("Data loaded from file: {data:?}");

      data
    },
    Err(e) => {
      println!("Failed to read from file: {e:?}");

      println!("Starting with an empty hashmap.");

      HashMap::new()
    },
  }
}

async fn read_data_from_file(
  file_path: &str
) -> io::Result<HashMap<String, Vec<u8>>> {
  let mut file = File::open(file_path).await?;

  let mut contents = String::new();

  file.read_to_string(&mut contents).await?;

  let data = serde_json::from_str(&contents)?;

  Ok(data)
}

async fn router(mut receiver: Receiver<RoutingMessage>) {
  let (mut key_value_sender, mut key_value_receiver) = channel(32);

  let mut key_value_handle = tokio::spawn(key_value_actor(key_value_receiver));

  let (heartbeat_sender, heartbeat_receiver) = channel(32);

  tokio::spawn(heartbeat_actor(heartbeat_receiver));

  while let Some(message) = receiver.recv().await {
    match message {
      RoutingMessage::KeyValue(message) => {
        let _ = key_value_sender.send(message).await;
      },
      RoutingMessage::Heartbeat(actor_type) => {
        let _ = heartbeat_sender.send(actor_type).await;
      },
      RoutingMessage::Reset(actor_type) => match actor_type {
        ActorType::KeyValue | ActorType::Writer => {
          let (new_key_value_sender, new_key_value_receiver) = channel(32);

          key_value_handle.abort();

          key_value_sender = new_key_value_sender;

          key_value_receiver = new_key_value_receiver;

          key_value_handle = tokio::spawn(key_value_actor(key_value_receiver));

          time::sleep(Duration::from_millis(100)).await;
        },
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

async fn writer_actor(
  mut receiver: Receiver<WriterLogMessage>
) -> io::Result<()> {
  let mut map: HashMap<String, Vec<u8>> = load_map("./data.json").await;

  let mut file: File = File::create("./data.json").await.unwrap();

  let timeout_duration: Duration = Duration::from_millis(200);

  let router_sender: Sender<RoutingMessage> =
    ROUTER_SENDER.get().unwrap().clone();

  loop {
    match time::timeout(timeout_duration, receiver.recv()).await {
      Ok(Some(message)) => {
        match message {
          WriterLogMessage::Delete(key) => {
            map.remove(&key);
          },
          WriterLogMessage::Get(sender) => {
            let _ = sender.send(map.clone());
          },
          WriterLogMessage::Set(key, value) => {
            map.insert(key, value);
          },
        }

        let contents: String = serde_json::to_string(&map).unwrap();

        file.set_len(0).await?;

        file.seek(std::io::SeekFrom::Start(0)).await?;

        file.write_all(contents.as_bytes()).await?;

        file.flush().await?;
      },
      Ok(None) => break,
      Err(_) => {
        router_sender
          .send(RoutingMessage::Heartbeat(ActorType::Writer))
          .await
          .unwrap();
      },
    }
  }

  Ok(())
}
