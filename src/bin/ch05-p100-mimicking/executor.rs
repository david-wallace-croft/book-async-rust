use ::std::{
  collections::VecDeque,
  ops::{Coroutine, CoroutineState},
  pin::Pin,
};

pub type CoroutineType = Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>;

#[derive(Default)]
pub struct Executor {
  pub coroutines: VecDeque<CoroutineType>,
}

impl Executor {
  pub fn add(
    &mut self,
    coroutine: CoroutineType,
  ) {
    self.coroutines.push_back(coroutine);
  }

  pub fn poll(&mut self) {
    println!("Polling {} coroutines", self.coroutines.len());

    let mut coroutine: CoroutineType = self.coroutines.pop_front().unwrap();

    match coroutine.as_mut().resume(()) {
      CoroutineState::Yielded(_) => self.coroutines.push_back(coroutine),
      CoroutineState::Complete(_) => {},
    }
  }
}
