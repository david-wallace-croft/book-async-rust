use super::key_value_message::KeyValueMessage;

pub enum RoutingMessage {
  KeyValue(KeyValueMessage),
}
