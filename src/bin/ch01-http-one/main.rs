use ::reqwest::Error;
use ::std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let url: &str = "https://jsonplaceholder.typicode.com/posts/1";

  let start_time: Instant = Instant::now();

  let _ = reqwest::get(url).await;

  let elapsed_time: Duration = start_time.elapsed();

  println!("Request took {} ms", elapsed_time.as_millis());

  Ok(())
}
