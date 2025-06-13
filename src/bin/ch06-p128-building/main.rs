use self::event_bus::EventBus;
use ::std::collections::{HashMap, VecDeque};
use ::std::mem;
use ::std::sync::Arc;
use ::std::thread;
use ::std::time::Duration;
use ::tokio::sync::MutexGuard;
use ::tokio::task::{self, JoinHandle};
use ::tokio::time;

pub mod event_bus;
pub mod event_handle;
#[tokio::main]
async fn main() {
  let event_bus: Arc<EventBus<f32>> = Default::default();

  let bus_one: Arc<EventBus<f32>> = event_bus.clone();

  let bus_two: Arc<EventBus<f32>> = event_bus.clone();

  let gb_bus_ref: Arc<EventBus<f32>> = event_bus.clone();

  let _gb: JoinHandle<()> =
    task::spawn(async { garbage_collector(gb_bus_ref).await });

  let one: JoinHandle<()> =
    task::spawn(async { consume_event_bus(bus_one).await });

  let two: JoinHandle<()> =
    task::spawn(async { consume_event_bus(bus_two).await });

  thread::sleep(Duration::from_secs(1));

  event_bus.send(1.).await;

  event_bus.send(2.).await;

  event_bus.send(3.).await;

  let _ = one.await;

  let _ = two.await;

  println!("{:?}", event_bus.chamber.lock().await);

  thread::sleep(Duration::from_secs(3));

  println!("{:?}", event_bus.chamber.lock().await);
}

async fn consume_event_bus(event_bus: Arc<EventBus<f32>>) {
  let handle: event_handle::EventHandle<'_, f32> = event_bus.subscribe().await;

  loop {
    let event: Option<f32> = handle.poll().await;

    if let Some(event) = event {
      println!("id: {} value: {}", handle.id, event);

      if event == 3.0 {
        break;
      }
    };
  }
}

async fn garbage_collector(event_bus: Arc<EventBus<f32>>) {
  loop {
    let mut chamber: MutexGuard<'_, HashMap<u32, VecDeque<f32>>> =
      event_bus.chamber.lock().await;

    let dead_ids: Vec<u32> = event_bus.dead_ids.lock().unwrap().clone();

    event_bus.dead_ids.lock().unwrap().clear();

    for id in dead_ids.iter() {
      chamber.remove(id);
    }

    mem::drop(chamber);

    time::sleep(Duration::from_secs(1)).await;
  }
}
