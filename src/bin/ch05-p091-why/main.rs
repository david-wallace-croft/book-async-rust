#![feature(coroutines)]
#![feature(coroutine_trait)]

use self::write_coroutine::WriteCoroutine;
use ::rand::prelude::*;
use ::std::fs::OpenOptions;
use ::std::io::{self, Write};
use ::std::ops::Coroutine;
use ::std::pin::Pin;
use ::std::time::Instant;

mod write_coroutine;

fn main() -> io::Result<()> {
  let mut rng: ThreadRng = rand::rng();

  let numbers: Vec<i32> = (0..200_000).map(|_| rng.random()).collect();

  let start = Instant::now();

  for &number in &numbers {
    if let Err(e) = append_number_to_file(number) {
      eprintln!("Failed to write to file: {e}");
    }
  }

  let duration = start.elapsed();

  println!("Time elapsed in file operations is: {duration:?}");

  let mut coroutine: WriteCoroutine = WriteCoroutine::new("numbers.txt")?;

  let start = Instant::now();

  for &number in &numbers {
    Pin::new(&mut coroutine).resume(number);
  }

  let duration = start.elapsed();

  println!("Time elapsed in file operations is: {duration:?}");

  Ok(())
}

fn append_number_to_file(n: i32) -> io::Result<()> {
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("numbers.txt")?;

  writeln!(file, "{n}")?;

  Ok(())
}
