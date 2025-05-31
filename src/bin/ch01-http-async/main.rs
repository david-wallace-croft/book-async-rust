use ::reqwest::{self, Error};
use ::serde::Deserialize;
use ::serde_json::Value;
use ::std::time::{Duration, Instant};
use ::tokio::time;

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct Response {
  args: Value,
  url: String,
}

async fn calculate_last_login() {
  time::sleep(Duration::from_secs(1)).await;

  println!("Logged in 2 days ago");
}

async fn fetch_data(seconds: u64) -> Result<Response, Error> {
  let request_url: String = format!("https://httpbin.org/delay/{seconds}");

  let response: reqwest::Response = reqwest::get(&request_url).await?;

  let delayed_response: Response = response.json().await?;

  Ok(delayed_response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let start_time: Instant = Instant::now();

  let data = fetch_data(5);

  let time_since = calculate_last_login();

  let (posts, _) = tokio::join!(data, time_since);

  let duration: Duration = start_time.elapsed();

  println!("Fetched {posts:?}");

  println!("Time taken: {duration:?}");

  Ok(())
}
