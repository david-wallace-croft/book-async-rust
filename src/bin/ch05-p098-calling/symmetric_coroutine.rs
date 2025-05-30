use ::std::pin::Pin;

pub trait SymmetricCoroutine {
  type Input;
  type Output;

  fn resume_with_input(
    self: Pin<&mut Self>,
    input: Self::Input,
  ) -> Self::Output;
}
