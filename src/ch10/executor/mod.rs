use super::task::Task;
use super::waker::create_raw_waker;
use ::std::{
  collections::VecDeque,
  future::Future,
  pin::Pin,
  sync::{Arc, mpsc::Receiver},
  task::{Context, Poll, Waker},
};

pub struct Executor {
  pub polling: VecDeque<Task>,
}

impl Executor {
  pub fn create_waker(&self) -> Arc<Waker> {
    Arc::new(unsafe { Waker::from_raw(create_raw_waker()) })
  }

  pub fn new() -> Self {
    Self {
      polling: Default::default(),
    }
  }

  pub fn poll(&mut self) {
    todo!()
  }

  pub fn spawn<F, T>(
    &mut self,
    future: F,
  ) -> Receiver<T>
  where
    F: Future<Output = T> + 'static + Send,
    T: Send + 'static,
  {
    todo!()
  }
}
