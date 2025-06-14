use ::std::process;
use ::std::thread;
use ::tokio::runtime::{Builder, Runtime};
use ::tokio::signal;

#[tokio::main]
async fn main() {
  thread::spawn(|| {
    let runtime: Runtime =
      Builder::new_multi_thread().enable_all().build().unwrap();

    runtime.block_on(async {
      println!("Hello, World!");
    });
  });

  let mut count: i32 = 0;

  loop {
    signal::ctrl_c().await.unwrap();

    println!("ctrl-c received!");

    count += 1;

    if count > 2 {
      process::exit(0);
    }
  }
}
