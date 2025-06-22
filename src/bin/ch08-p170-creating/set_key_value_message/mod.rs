use ::tokio::sync::oneshot;

pub struct SetKeyValueMessage {
  pub key: String,
  pub response: oneshot::Sender<()>,
  pub value: Vec<u8>,
}
