pub trait AsyncProcess<X, Y, Z> {
  fn spawn(
    &self,
    input: X,
  ) -> Result<Y, String>;

  fn get_result(
    &self,
    key: Y,
  ) -> Result<Z, String>;
}
