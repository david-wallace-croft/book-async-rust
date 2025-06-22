use super::{
  delete_key_value_message::DeleteKeyValueMessage,
  get_key_value_message::GetKeyValueMessage,
  set_key_value_message::SetKeyValueMessage,
};

pub enum KeyValueMessage {
  Delete(DeleteKeyValueMessage),
  Get(GetKeyValueMessage),
  Set(SetKeyValueMessage),
}
