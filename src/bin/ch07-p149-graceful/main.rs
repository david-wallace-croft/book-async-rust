use ::std::process;
use ::std::{thread, time::Duration};
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

  let mut count: i32 = 0;

  loop {
    thread::sleep(Duration::from_secs(5));

    signal::ctrl_c().await.unwrap();

    println!("ctrl-c received!");

    count += 1;

    if count > 2 {
      process::exit(0);
    }
  }
}
