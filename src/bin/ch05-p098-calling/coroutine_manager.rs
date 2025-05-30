#![expect(dead_code)]

use super::read_coroutine::ReadCoroutine;
use super::write_coroutine::WriteCoroutine;
use ::std::io;
use ::std::ops::{Coroutine, CoroutineState};
use ::std::pin::Pin;

pub struct CoroutineManager {
  pub reader: ReadCoroutine,
  pub writer: WriteCoroutine,
}

impl CoroutineManager {
  pub fn new(
    read_path: &str,
    write_path: &str,
  ) -> io::Result<Self> {
    let reader: ReadCoroutine = ReadCoroutine::new(read_path)?;

    let writer: WriteCoroutine = WriteCoroutine::new(write_path)?;

    Ok(Self {
      reader,
      writer,
    })
  }

  pub fn run(&mut self) {
    let mut read_pin: Pin<&mut ReadCoroutine> = Pin::new(&mut self.reader);

    let mut write_pin: Pin<&mut WriteCoroutine> = Pin::new(&mut self.writer);

    loop {
      let coroutine_state: CoroutineState<i32, ()> =
        read_pin.as_mut().resume(());

      match coroutine_state {
        CoroutineState::Yielded(number) => {
          write_pin.as_mut().resume(number);
        },
        CoroutineState::Complete(()) => break,
      }
    }
  }
}
