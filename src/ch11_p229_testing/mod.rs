#[cfg(test)]
mod tests {
  use ::tokio::runtime::{Builder, Runtime};
  use ::tokio::sync::mpsc;
  use ::tokio::task::JoinHandle;
  use ::tokio::time::error::Elapsed;
  use ::tokio::time::{Duration, timeout};

  #[test]
  fn test_channel_capacity() {
    let runtime: Runtime =
      Builder::new_current_thread().enable_all().build().unwrap();

    let (sender, mut receiver) = mpsc::channel::<i32>(5);

    let sender: JoinHandle<()> = runtime.spawn(async move {
      for i in 0..10 {
        sender.send(i).await.expect("Failed to send message");
      }
    });

    let _receiver: JoinHandle<()> = runtime.spawn(async move {
      let mut i: i32 = 0;

      while let Some(msg) = receiver.recv().await {
        assert_eq!(msg, i);

        i += 1;

        println!("Got message: {}", msg);
      }
    });

    let result: Result<(), Elapsed> = runtime.block_on(async {
      timeout(Duration::from_secs(5), async {
        sender.await.unwrap();
      })
      .await
    });

    assert!(
      result.is_ok(),
      "A potential filled channel is not handled correctly"
    );
  }
}
