use ::core::task::Poll;
use ::std::future::Future;
use ::std::pin::Pin;
use ::std::sync::{Arc, Mutex};
use ::std::task::Context;
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

struct CounterFuture {
  counter_type: CounterType,
  data_reference: Arc<Mutex<SharedData>>,
  count: u32,
}

impl Future for CounterFuture {
  type Output = u32;

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    std::thread::sleep(Duration::from_secs(1));

    let mut guard: std::sync::MutexGuard<'_, SharedData> =
      match self.data_reference.try_lock() {
        Ok(guard) => guard,
        Err(error) => {
          println!("error for {:?}: {}", self.counter_type, error);

          context.waker().wake_by_ref();

          return Poll::Pending;
        },
      };

    let value: &mut SharedData = &mut guard;

    match self.counter_type {
      CounterType::Decrement => {
        value.decrement();

        println!("After decrement: {}", value.counter);
      },
      CounterType::Increment => {
        value.increment();

        println!("After increment: {}", value.counter);
      },
    }

    std::mem::drop(guard);

    self.count += 1;

    if self.count < 3 {
      context.waker().wake_by_ref();

      return Poll::Pending;
    }

    Poll::Ready(self.count)
  }
}

#[tokio::main]
async fn main() {
  let shared_data: Arc<Mutex<SharedData>> = Default::default();

  let counter_one: CounterFuture = CounterFuture {
    counter_type: CounterType::Increment,
    data_reference: shared_data.clone(),
    count: 0,
  };

  let counter_two: CounterFuture = CounterFuture {
    counter_type: CounterType::Decrement,
    data_reference: shared_data.clone(),
    count: 0,
  };

  let handle_one: JoinHandle<u32> = tokio::task::spawn(counter_one);

  let handle_two: JoinHandle<u32> = tokio::task::spawn(counter_two);

  let _result: (
    Result<u32, tokio::task::JoinError>,
    Result<u32, tokio::task::JoinError>,
  ) = tokio::join!(handle_one, handle_two);
}
