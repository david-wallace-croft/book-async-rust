use ::std::cell::UnsafeCell;
use ::std::collections::HashMap;
use ::std::time::Duration;
use ::tokio::task::JoinHandle;
use ::tokio::time;
use ::tokio_util::task::LocalPoolHandle;

thread_local! {
    pub static COUNTER: UnsafeCell<HashMap<u32, u32>> = Default::default();
}

#[tokio::main]
async fn main() {
  let pool: LocalPoolHandle = LocalPoolHandle::new(1);

  let sequence: [u32; 5] = [
    1, 2, 3, 4, 5,
  ];

  let repeated_sequence: Vec<_> =
    sequence.iter().cycle().take(500_000).cloned().collect();

  let mut futures: Vec<JoinHandle<()>> = Vec::new();

  for number in repeated_sequence {
    futures.push(pool.spawn_pinned(move || async move {
      something(number).await;
      something(number).await
    }));
  }

  for i in futures {
    let _ = i.await.unwrap();
  }

  let _ = pool
    .spawn_pinned(|| async { print_statement().await })
    .await
    .unwrap();
}

async fn print_statement() {
  COUNTER.with(|counter: &UnsafeCell<HashMap<u32, u32>>| {
    let counter: &mut HashMap<u32, u32> = unsafe { &mut *counter.get() };

    println!("Counter: {:?}", counter);
  });
}

async fn something(number: u32) {
  time::sleep(Duration::from_secs(number as u64)).await;

  COUNTER.with(|counter: &UnsafeCell<HashMap<u32, u32>>| {
    let counter: &mut HashMap<u32, u32> = unsafe { &mut *counter.get() };

    match counter.get_mut(&number) {
      Some(count) => {
        let placeholder: u32 = *count + 1;

        *count = placeholder;
      },
      None => {
        counter.insert(number, 1);
      },
    }
  });
}
