use std::{
  pin::Pin,
  task::{Context, Poll},
  thread,
  time::Duration,
};

#[derive(Clone, Copy, Debug)]
pub struct BackgroundProcess;

impl Future for BackgroundProcess {
  type Output = ();

  fn poll(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    println!("Starting background process poll...");

    thread::sleep(Duration::from_secs(1));

    context.waker().wake_by_ref();

    println!("Finished background process poll.");

    Poll::Pending
  }
}
