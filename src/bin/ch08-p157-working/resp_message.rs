use tokio::sync::oneshot;

pub struct RespMessage {
  pub responder: oneshot::Sender<usize>,
  pub value: usize,
}
