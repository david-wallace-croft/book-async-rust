use self::logging_future::LoggingFuture;

mod logging;
mod logging_future;

#[tokio::main]
async fn main() {
  let logged_future = LoggingFuture {
    inner: my_async_function(),
  };

  let result: String = logged_future.await;

  println!("{result}");
}

async fn my_async_function() -> String {
  "Result of async computation".to_string()
}
