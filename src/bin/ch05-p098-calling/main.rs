#![feature(coroutines)]
#![feature(coroutine_trait)]

use self::read_coroutine::ReadCoroutine;
use self::symmetric_coroutine::SymmetricCoroutine;
use self::write_coroutine::WriteCoroutine;
use ::std::io;
use ::std::pin::Pin;

mod coroutine_manager;
mod read_coroutine;
mod symmetric_coroutine;
mod write_coroutine;

fn main() -> io::Result<()> {
  let mut reader: ReadCoroutine = ReadCoroutine::new("numbers.txt")?;

  let mut writer: WriteCoroutine = WriteCoroutine::new("output.txt")?;

  loop {
    let number: Option<i32> = Pin::new(&mut reader).resume_with_input(());

    if let Some(num) = number {
      Pin::new(&mut writer).resume_with_input(num);
    } else {
      break;
    }
  }

  Ok(())
}
