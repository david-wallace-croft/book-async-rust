use ::std::time::Duration;
use ::tokio::time::timeout;

#[tokio::main]
async fn main() {
  let duration: Duration = Duration::from_secs(3);

  let result: Result<&'static str, tokio::time::error::Elapsed> =
    timeout(duration, slow_task()).await;

  match result {
    Ok(value) => println!("Task completed successfully: {value}"),
    Err(_) => println!("Task timed out"),
  }
}

async fn slow_task() -> &'static str {
  tokio::time::sleep(Duration::from_secs(10)).await;

  "Slow task completed"
}
