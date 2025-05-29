#![feature(coroutines)]
#![feature(coroutine_trait)]

use coroutine_manager::CoroutineManager;

mod coroutine_manager;
mod read_coroutine;
mod write_coroutine;

fn main() {
  let mut manager: CoroutineManager =
    CoroutineManager::new("numbers.txt", "output.txt").unwrap();

  manager.run();
}
