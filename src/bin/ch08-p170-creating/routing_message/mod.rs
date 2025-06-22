use super::actor_type::ActorType;
use super::key_value_message::KeyValueMessage;

pub enum RoutingMessage {
  Heartbeat(ActorType),
  KeyValue(KeyValueMessage),
  Reset(ActorType),
}
