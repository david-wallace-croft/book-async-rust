use ::std::fs::{File, OpenOptions};
use ::std::io::{self, Write};
use ::std::ops::{Coroutine, CoroutineState};
use ::std::pin::Pin;

use crate::symmetric_coroutine::SymmetricCoroutine;

pub struct WriteCoroutine {
  pub file_handle: File,
}

impl WriteCoroutine {
  pub fn new(path: &str) -> io::Result<Self> {
    let file_handle: File =
      OpenOptions::new().create(true).append(true).open(path)?;

    Ok(Self {
      file_handle,
    })
  }
}

impl Coroutine<i32> for WriteCoroutine {
  type Yield = ();

  type Return = ();

  fn resume(
    mut self: Pin<&mut Self>,
    arg: i32,
  ) -> CoroutineState<Self::Yield, Self::Return> {
    writeln!(self.file_handle, "{arg}").unwrap();

    CoroutineState::Yielded(())
  }
}

impl SymmetricCoroutine for WriteCoroutine {
  type Input = i32;

  type Output = ();

  fn resume_with_input(
    mut self: Pin<&mut Self>,
    input: Self::Input,
  ) -> Self::Output {
    writeln!(self.file_handle, "{input}").unwrap();
  }
}
