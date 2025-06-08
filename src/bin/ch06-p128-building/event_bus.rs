use super::event_handle::EventHandle;
use ::std::collections::{HashMap, VecDeque};
use ::std::sync::atomic::{AtomicU32, Ordering};
use ::std::sync::{Arc, Mutex};
use ::tokio::sync::{Mutex as AsyncMutex, MutexGuard};

pub struct EventBus<T: Clone + Send> {
  pub chamber: AsyncMutex<HashMap<u32, VecDeque<T>>>,
  pub count: AtomicU32,
  pub dead_ids: Mutex<Vec<u32>>,
}

impl<T: Clone + Send> EventBus<T> {
  pub async fn subscribe(&self) -> EventHandle<T> {
    let mut chamber = self.chamber.lock().await;

    let id: u32 = self.count.fetch_add(1, Ordering::SeqCst);

    chamber.insert(id, VecDeque::new());

    EventHandle {
      id,
      event_bus: Arc::new(self),
    }
  }

  pub fn unsubscribe(
    &self,
    id: u32,
  ) {
    self.dead_ids.lock().unwrap().push(id);
  }

  pub async fn poll(
    &self,
    id: u32,
  ) -> Option<T> {
    let mut chamber: MutexGuard<'_, HashMap<u32, VecDeque<T>>> =
      self.chamber.lock().await;

    let queue: &mut VecDeque<T> = chamber.get_mut(&id).unwrap();

    queue.pop_front()
  }

  pub async fn send(
    &self,
    event: T,
  ) {
    let mut chamber: MutexGuard<'_, HashMap<u32, VecDeque<T>>> =
      self.chamber.lock().await;

    for (_, value) in chamber.iter_mut() {
      value.push_back(event.clone());
    }
  }
}

impl<T: Clone + Send> Default for EventBus<T> {
  fn default() -> Self {
    Self {
      chamber: Default::default(),
      count: Default::default(),
      dead_ids: Default::default(),
    }
  }
}
