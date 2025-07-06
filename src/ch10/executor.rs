use super::task::Task;
use super::waker::create_raw_waker;
use ::std::{
  collections::VecDeque,
  future::Future,
  pin::Pin,
  sync::{
    Arc,
    mpsc::{self, Receiver},
  },
  task::{Context, Poll, Waker},
};

#[derive(Default)]
pub struct Executor {
  pub polling: VecDeque<Task>,
}

impl Executor {
  pub fn create_waker(&self) -> Arc<Waker> {
    Arc::new(unsafe { Waker::from_raw(create_raw_waker()) })
  }

  pub fn poll(&mut self) {
    let mut task: Task = match self.polling.pop_front() {
      Some(task) => task,
      None => return,
    };

    let waker: Arc<Waker> = task.waker.clone();

    let context: &mut Context<'_> = &mut Context::from_waker(&waker);

    match task.future.as_mut().poll(context) {
      Poll::Ready(()) => {},
      Poll::Pending => {
        self.polling.push_back(task);
      },
    }
  }

  pub fn spawn<F, T>(
    &mut self,
    future: F,
  ) -> Receiver<T>
  where
    F: Future<Output = T> + 'static + Send,
    T: Send + 'static,
  {
    let (tx, rx) = mpsc::channel();

    let future: Pin<Box<dyn Future<Output = ()> + Send>> =
      Box::pin(async move {
        let result: T = future.await;

        let _ = tx.send(result);
      });

    let task: Task = Task {
      future,
      waker: self.create_waker(),
    };

    self.polling.push_back(task);

    rx
  }
}
