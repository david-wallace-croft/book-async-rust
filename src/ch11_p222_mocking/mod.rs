use self::async_process::AsyncProcess;

mod async_process;

pub async fn do_something<T>(
  async_handle: T,
  input: i32,
) -> Result<i32, String>
where
  T: AsyncProcess<i32, i32> + Send + Sync + 'static,
{
  println!("something is happening");

  let result: i32 = async_handle.get_result(input).await?;

  if result > 10 {
    return Err("result is too big".to_string());
  }

  if result == 8 {
    return Ok(result * 2);
  }

  Ok(result * 3)
}

#[cfg(test)]
mod get_team_processes_tests {
  use super::*;
  use ::mockall::mock;
  use ::mockall::predicate::*;
  use ::std::boxed::Box;

  mock! {
    DatabaseHandler {}

    impl AsyncProcess<i32, i32> for DatabaseHandler {
      fn get_result(&self, key: i32)
        -> impl Future<Output = Result<i32, String>> + Send + 'static;
    }
  }

  #[test]
  fn do_something_fail() {
    let mut handle: MockDatabaseHandler = MockDatabaseHandler::new();

    handle
      .expect_get_result()
      .with(eq(4))
      .returning(|_| Box::pin(async move { Ok(11) }));

    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap();

    let outcome: Result<i32, String> =
      runtime.block_on(do_something(handle, 4));

    assert_eq!(outcome, Err("result is too big".into()));
  }
}
