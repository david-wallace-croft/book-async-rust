use super::future_order_label::FutureOrderLabel;
use super::future_type::FutureType;
use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};

pub struct CounterFuture {
  pub count: u32,
  #[expect(dead_code)]
  pub order: FutureType,
}

impl CounterFuture {
  pub fn new(order: FutureType) -> Self {
    Self {
      count: 0,
      order,
    }
  }
}

impl Future for CounterFuture {
  type Output = u32;

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    self.count += 1;

    println!("CounterFuture::poll() count: {}", self.count);

    println!("Starting CounterFuture::poll() sleep...");

    std::thread::sleep(Duration::from_secs(1));

    println!("Finished CounterFuture::poll() sleep.");

    if self.count < 3 {
      context.waker().wake_by_ref();

      return Poll::Pending;
    }

    Poll::Ready(self.count)
  }
}

impl FutureOrderLabel for CounterFuture {
  fn get_order(&self) -> FutureType {
    self.order
  }
}
