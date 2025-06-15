use ::std::cell::UnsafeCell;
use ::std::collections::HashMap;
use ::std::sync::LazyLock;
use ::std::time::Duration;
use ::tokio::signal;
use ::tokio::task::JoinHandle;
use ::tokio::time;
use ::tokio_util::task::LocalPoolHandle;

thread_local! {
    pub static COUNTER: UnsafeCell<HashMap<u32, u32>> = Default::default();
}

static RUNTIME: LazyLock<LocalPoolHandle> =
  LazyLock::new(|| LocalPoolHandle::new(4));

#[tokio::main]
async fn main() {
  let _handle: JoinHandle<()> = tokio::spawn(async {
    let sequence: [u32; 5] = [
      1, 2, 3, 4, 5,
    ];

    let repeated_sequence: Vec<_> =
      sequence.iter().cycle().take(500_000).cloned().collect();

    let mut futures: Vec<JoinHandle<()>> = Default::default();

    for number in repeated_sequence {
      futures.push(RUNTIME.spawn_pinned(move || async move {
        something(number).await;
        something(number).await
      }));
    }

    for i in futures {
      i.await.unwrap();
    }

    println!("All futures completed");
  });

  signal::ctrl_c().await.unwrap();

  println!("ctrl-c received!");

  let complete_counter: HashMap<u32, u32> = get_complete_count().await;

  println!("Complete counter: {complete_counter:?}");
}

fn extract_data_from_thread() -> HashMap<u32, u32> {
  let mut extracted_counter: HashMap<u32, u32> = Default::default();

  COUNTER.with(|counter: &UnsafeCell<HashMap<u32, u32>>| {
    let counter: &mut HashMap<u32, u32> = unsafe { &mut *counter.get() };

    extracted_counter = counter.clone();
  });

  extracted_counter
}

async fn get_complete_count() -> HashMap<u32, u32> {
  let mut complete_counter: HashMap<u32, u32> = Default::default();

  let mut extracted_counters: Vec<JoinHandle<HashMap<u32, u32>>> =
    Default::default();

  for i in 0..4 {
    extracted_counters.push(
      RUNTIME
        .spawn_pinned_by_idx(|| async move { extract_data_from_thread() }, i),
    );
  }

  for counter_future in extracted_counters {
    let extracted_counter: HashMap<u32, u32> =
      counter_future.await.unwrap_or_default();

    for (key, count) in extracted_counter {
      *complete_counter.entry(key).or_insert(0) += count;
    }
  }

  complete_counter
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
