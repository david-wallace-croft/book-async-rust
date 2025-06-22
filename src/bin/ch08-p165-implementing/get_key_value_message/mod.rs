use ::tokio::sync::oneshot;

pub struct GetKeyValueMessage {
  pub key: String,
  pub response: oneshot::Sender<Option<Vec<u8>>>,
}
