#![feature(coroutines)]
#![feature(coroutine_trait)]

use self::read_coroutine::ReadCoroutine;
use self::write_coroutine::WriteCoroutine;
use ::rand::prelude::*;
use ::std::io::{self};
use ::std::ops::{Coroutine, CoroutineState};
use ::std::pin::Pin;
use ::std::time::{Duration, Instant};

mod read_coroutine;
mod write_coroutine;

fn main() -> io::Result<()> {
  let mut rng: ThreadRng = rand::rng();

  let numbers: Vec<i32> = (0..10).map(|_| rng.random()).collect();

  let mut coroutine: WriteCoroutine = WriteCoroutine::new("numbers.txt")?;

  let start: Instant = Instant::now();

  for &number in &numbers {
    Pin::new(&mut coroutine).resume(number);
  }

  let duration: Duration = start.elapsed();

  println!("Time elapsed in file operations is: {:?}", duration);

  let start: Instant = Instant::now();

  let mut coroutine: ReadCoroutine = ReadCoroutine::new("numbers.txt")?;

  loop {
    let pointer: &mut ReadCoroutine = &mut coroutine;

    let pin: Pin<&mut ReadCoroutine> = Pin::new(pointer);

    let coroutine_state: CoroutineState<_, ()> = pin.resume(());

    match coroutine_state {
      CoroutineState::Yielded(number) => println!("{:?}", number),
      CoroutineState::Complete(()) => break,
    }
  }

  let duration: Duration = start.elapsed();

  println!("Time elapsed in file operations is: {:?}", duration);

  Ok(())
}
