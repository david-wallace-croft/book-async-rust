use reqwest::{Error, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let url: &str = "https://jsonplaceholder.typicode.com/posts/1";

  let response: Response = reqwest::get(url).await?;

  if response.status().is_success() {
    let body: String = response.text().await?;

    println!("{body}");
  } else {
    println!(
      "Failed to get a valid response.  Status: {}",
      response.status()
    );
  }

  Ok(())
}
