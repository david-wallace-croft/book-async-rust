use ::std::pin::Pin;
use ::std::task::Context;
use ::std::task::{Poll, Waker};
use ::std::thread::sleep as standard_sleep;
use ::std::time::Duration;

#[derive(Default)]
struct CounterFuture {
  count: u32,
}

impl Future for CounterFuture {
  type Output = u32;

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    self.count += 1;

    println!("Polling with result: {}", self.count);

    standard_sleep(Duration::from_secs(1));

    if self.count < 5 {
      let waker: &Waker = context.waker();

      waker.wake_by_ref();

      return Poll::Pending;
    }

    Poll::Ready(self.count)
  }
}

#[tokio::main]
async fn main() {
  let counter_one: CounterFuture = Default::default();

  let counter_two: CounterFuture = Default::default();

  let handle_one: tokio::task::JoinHandle<u32> =
    tokio::task::spawn(counter_one);

  let handle_two: tokio::task::JoinHandle<u32> =
    tokio::task::spawn(counter_two);

  let _result: (
    Result<u32, tokio::task::JoinError>,
    Result<u32, tokio::task::JoinError>,
  ) = tokio::join!(handle_one, handle_two);
}
