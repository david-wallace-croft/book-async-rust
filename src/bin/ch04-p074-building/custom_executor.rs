use crate::spawn_task;

pub struct CustomExecutor;

impl<F: Future + Send + 'static> hyper::rt::Executor<F> for CustomExecutor {
  fn execute(
    &self,
    fut: F,
  ) {
    spawn_task!(async {
      println!("Starting request...");
      fut.await;
      println!("Finished request.");
    })
    .detach();
  }
}
