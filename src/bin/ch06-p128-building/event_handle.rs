use super::event_bus::EventBus;
use std::sync::Arc;

pub struct EventHandle<'a, T: Clone + Send> {
  pub event_bus: Arc<&'a EventBus<T>>,
  pub id: u32,
}

impl<'a, T: Clone + Send> EventHandle<'a, T> {
  pub async fn poll(&self) -> Option<T> {
    self.event_bus.poll(self.id).await
  }
}

impl<'a, T: Clone + Send> Drop for EventHandle<'a, T> {
  fn drop(&mut self) {
    self.event_bus.unsubscribe(self.id);
  }
}
