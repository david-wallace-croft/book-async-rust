use ::std::sync::Arc;
use ::tokio::task::JoinHandle;
use ::tokio::time::Duration;

#[derive(Debug)]
enum CounterType {
  Decrement,
  Increment,
}

#[derive(Default)]
struct SharedData {
  counter: i32,
}

impl SharedData {
  fn decrement(&mut self) {
    self.counter -= 1;
  }

  fn increment(&mut self) {
    self.counter += 1;
  }
}

async fn count(
  count: u32,
  data: Arc<tokio::sync::Mutex<SharedData>>,
  counter_type: CounterType,
) -> u32 {
  for _ in 0..count {
    let mut data: tokio::sync::MutexGuard<'_, SharedData> = data.lock().await;

    match counter_type {
      CounterType::Decrement => {
        data.decrement();

        println!("After decrement: {}", data.counter);
      },
      CounterType::Increment => {
        data.increment();

        println!("After increment: {}", data.counter);
      },
    }

    std::mem::drop(data);

    std::thread::sleep(Duration::from_secs(1));
  }

  count
}

#[tokio::main]
async fn main() {
  let shared_data: Arc<tokio::sync::Mutex<SharedData>> = Default::default();

  let shared_two: Arc<tokio::sync::Mutex<SharedData>> = shared_data.clone();

  let counter_one =
    async move { count(3, shared_data, CounterType::Increment).await };

  let counter_two =
    async move { count(3, shared_two, CounterType::Decrement).await };

  let handle_one: JoinHandle<u32> = tokio::task::spawn(counter_one);

  let handle_two: JoinHandle<u32> = tokio::task::spawn(counter_two);

  let _result: (
    Result<u32, tokio::task::JoinError>,
    Result<u32, tokio::task::JoinError>,
  ) = tokio::join!(handle_one, handle_two);
}
