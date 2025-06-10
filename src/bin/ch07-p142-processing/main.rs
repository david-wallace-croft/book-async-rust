use ::std::cell::RefCell;
use ::std::{thread, time::Duration};
use ::tokio::task::JoinHandle;
use ::tokio_util::task::LocalPoolHandle;

thread_local! {
  pub static COUNTER: RefCell<u32> = RefCell::new(1);
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let pool: LocalPoolHandle = LocalPoolHandle::new(1);

  let one: JoinHandle<u32> = pool.spawn_pinned(|| async {
    println!("one");

    something(1).await
  });

  let two: JoinHandle<u32> = pool.spawn_pinned(|| async {
    println!("two");

    something(2).await
  });

  let three: JoinHandle<u32> = pool.spawn_pinned(|| async {
    println!("three");

    something(3).await
  });

  let result = async {
    let one: u32 = one.await.unwrap();
    let two: u32 = two.await.unwrap();
    let three: u32 = three.await.unwrap();

    one + two + three
  };

  println!("result: {}", result.await);
}

async fn something(number: u32) -> u32 {
  thread::sleep(Duration::from_secs(3));

  COUNTER.with(|counter: &RefCell<u32>| {
    *counter.borrow_mut() += 1;

    println!("Counter: {} for: {}", *counter.borrow(), number);
  });

  number
}
