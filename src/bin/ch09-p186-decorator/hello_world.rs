use super::greeting::Greeting;

pub struct HelloWorld;

impl Greeting for HelloWorld {
  fn greet(&self) -> String {
    "Hello, World!".into()
  }
}
