#[cfg(feature = "logging_decorator")]
use self::excited_greeting::ExcitedGreeting;

#[cfg(not(feature = "logging_decorator"))]
use self::greeting::Greeting;

use self::hello_world::HelloWorld;

#[cfg(feature = "logging_decorator")]
mod excited_greeting;

mod greeting;
mod hello_world;

fn main() {
  #[cfg(feature = "logging_decorator")]
  let hello: ExcitedGreeting<HelloWorld> = ExcitedGreeting {
    inner: HelloWorld,
  };

  #[cfg(not(feature = "logging_decorator"))]
  let hello: HelloWorld = HelloWorld;

  println!("{}", hello.greet());
}
