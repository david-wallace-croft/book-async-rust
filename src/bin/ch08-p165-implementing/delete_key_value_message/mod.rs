use ::tokio::sync::oneshot;

pub struct DeleteKeyValueMessage {
  pub key: String,
  pub response: oneshot::Sender<()>,
}
