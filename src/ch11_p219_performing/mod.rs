use self::async_process::AsyncProcess;

mod async_process;

pub fn do_something<T>(
  async_handle: T,
  input: i32,
) -> Result<i32, String>
where
  T: AsyncProcess<i32, String, i32>,
{
  let key: String = async_handle.spawn(input)?;

  println!("something is happening");

  let result: i32 = async_handle.get_result(key)?;

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
  use mockall::mock;
  use mockall::predicate::*;

  mock! {
    DatabaseHandler {}

    impl AsyncProcess<i32, String, i32> for DatabaseHandler {
      fn spawn(&self, input: i32) -> Result<String, String>;

      fn get_result(&self, key: String) -> Result<i32, String>;
    }
  }

  #[test]
  fn do_something_fail() {
    let mut handle: MockDatabaseHandler = MockDatabaseHandler::new();

    handle
      .expect_spawn()
      .with(eq(4))
      .returning(|_| Ok("test_key".into()));

    handle
      .expect_get_result()
      .with(eq("test_key".to_string()))
      .returning(|_| Ok(11));

    let outcome: Result<i32, String> = do_something(handle, 4);

    assert_eq!(outcome, Err("result is too big".into()));
  }
}
