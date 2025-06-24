use super::key_value_message::KeyValueMessage;
use ::std::collections::HashMap;
use ::tokio::sync::oneshot;

pub enum WriterLogMessage {
  Delete(String),
  Get(oneshot::Sender<HashMap<String, Vec<u8>>>),
  Set(String, Vec<u8>),
}

impl WriterLogMessage {
  pub fn from_key_value_message(
    message: &KeyValueMessage
  ) -> Option<WriterLogMessage> {
    match message {
      KeyValueMessage::Delete(delete_key_value_message) => Some(
        WriterLogMessage::Delete(delete_key_value_message.key.clone()),
      ),
      KeyValueMessage::Get(_get_key_value_message) => None,
      KeyValueMessage::Set(set_key_value_message) => {
        Some(WriterLogMessage::Set(
          set_key_value_message.key.clone(),
          set_key_value_message.value.clone(),
        ))
      },
    }
  }
}
