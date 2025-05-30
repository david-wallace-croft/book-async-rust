use super::symmetric_coroutine::SymmetricCoroutine;
use ::std::fs::File;
use ::std::io::{self, BufRead, BufReader};
use ::std::ops::{Coroutine, CoroutineState};
use ::std::pin::Pin;

pub struct ReadCoroutine {
  pub lines: io::Lines<BufReader<File>>,
}

impl ReadCoroutine {
  pub fn new(path: &str) -> io::Result<Self> {
    let file = File::open(path)?;

    let reader: BufReader<File> = BufReader::new(file);

    let lines: io::Lines<BufReader<File>> = reader.lines();

    Ok(Self {
      lines,
    })
  }
}

impl Coroutine<()> for ReadCoroutine {
  type Yield = i32;

  type Return = ();

  fn resume(
    mut self: Pin<&mut Self>,
    _arg: (),
  ) -> CoroutineState<Self::Yield, Self::Return> {
    let line_option: Option<Result<String, io::Error>> = self.lines.next();

    match line_option {
      Some(Ok(line)) => {
        if let Ok(number) = line.parse::<i32>() {
          CoroutineState::Yielded(number)
        } else {
          CoroutineState::Complete(())
        }
      },
      Some(Err(_)) | None => CoroutineState::Complete(()),
    }
  }
}

impl SymmetricCoroutine for ReadCoroutine {
  type Input = ();

  type Output = Option<i32>;

  fn resume_with_input(
    mut self: Pin<&mut Self>,
    _input: Self::Input,
  ) -> Self::Output {
    if let Some(Ok(line)) = self.lines.next() {
      line.parse::<i32>().ok()
    } else {
      None
    }
  }
}
