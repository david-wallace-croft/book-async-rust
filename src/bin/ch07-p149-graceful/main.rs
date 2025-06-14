use ::std::process;
use ::tokio::signal;

#[tokio::main]
async fn main() {
  tokio::spawn(cleanup());

  loop {
    // Is this 100% CPU?
  }
}

async fn cleanup() {
  println!("cleanup background task started");

  let mut count = 0;

  loop {
    signal::ctrl_c().await.unwrap();

    println!("ctrl-c received!");

    count += 1;

    if count > 2 {
      process::exit(0);
    }
  }
}
