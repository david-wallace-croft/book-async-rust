use super::greeting::Greeting;

pub struct ExcitedGreeting<T> {
  pub inner: T,
}

impl<T> ExcitedGreeting<T> {
  pub fn greet(&self) -> String
  where
    T: Greeting,
  {
    let mut greeting: String = self.inner.greet();

    greeting.push_str(" I'm so excited to be in Rust!");

    greeting
  }
}
