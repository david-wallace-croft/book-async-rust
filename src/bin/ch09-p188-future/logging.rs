pub trait Logging {
  fn log(&self);
}

impl<F: Future> Logging for F {
  fn log(&self) {
    println!("Polling the future!");
  }
}
