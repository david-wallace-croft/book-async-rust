use ::std::{future::Future, pin::Pin, sync::Arc, task::Waker};

pub struct Task {
  pub future: Pin<Box<dyn Future<Output = ()> + Send>>,
  pub waker: Arc<Waker>,
}
