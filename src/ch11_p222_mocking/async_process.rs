use ::std::future::Future;

pub trait AsyncProcess<X, Z> {
  fn get_result(
    &self,
    key: X,
  ) -> impl Future<Output = Result<Z, String>> + Send + 'static;
}
