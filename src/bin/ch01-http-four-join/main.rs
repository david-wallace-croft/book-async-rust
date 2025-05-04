use ::reqwest::Error;
use ::std::time::{Duration, Instant};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
  let url: &str = "https://jsonplaceholder.typicode.com/posts/1";

  let start_time: Instant = Instant::now();

  let first = reqwest::get(url);

  let second = reqwest::get(url);

  let third = reqwest::get(url);

  let fourth = reqwest::get(url);

  let (_, _, _, _) = tokio::join!(first, second, third, fourth);

  let elapsed_time: Duration = start_time.elapsed();

  println!("Request took {} ms", elapsed_time.as_millis());

  Ok(())
}
