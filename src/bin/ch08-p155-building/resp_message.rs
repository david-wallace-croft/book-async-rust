use tokio::sync::oneshot;

pub struct RespMessage {
  pub responder: oneshot::Sender<i64>,
  pub value: i64,
}
