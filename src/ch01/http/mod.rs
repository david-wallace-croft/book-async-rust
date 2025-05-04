use ::reqwest::Error;
use ::std::time::Instant;

#[tokio::main]
pub async fn run() -> Result<(), Error> {
  // run_one().await
  // run_four().await
  run_four_join().await
}

async fn run_one() -> Result<(), Error> {
  let url = "https://jsonplaceholder.typicode.com/posts/1";

  let start_time = Instant::now();

  let _ = reqwest::get(url).await?;

  let elapsed_time = start_time.elapsed();

  println!("Request took {} ms", elapsed_time.as_millis());

  Ok(())
}

async fn run_four() -> Result<(), Error> {
  let url = "https://jsonplaceholder.typicode.com/posts/1";

  let start_time = Instant::now();

  let first = reqwest::get(url);

  let second = reqwest::get(url);

  let third = reqwest::get(url);

  let fourth = reqwest::get(url);

  let _ = first.await?;

  let _ = second.await?;

  let _ = third.await?;

  let _ = fourth.await?;

  let elapsed_time = start_time.elapsed();

  println!("Request took {} ms", elapsed_time.as_millis());

  Ok(())
}

async fn run_four_join() -> Result<(), Error> {
  let url = "https://jsonplaceholder.typicode.com/posts/1";

  let start_time = Instant::now();

  let first = reqwest::get(url);

  let second = reqwest::get(url);

  let third = reqwest::get(url);

  let fourth = reqwest::get(url);

  let (_, _, _, _) = tokio::join!(first, second, third, fourth);

  let elapsed_time = start_time.elapsed();

  println!("Request took {} ms", elapsed_time.as_millis());

  Ok(())
}
