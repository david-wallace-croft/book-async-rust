use ::std::thread::sleep as standard_sleep;
use ::std::time::{Duration, Instant};
use ::tokio::time::sleep as tokio_sleep;

struct Task {
  afore: &'static str,
  after: &'static str,
  delay: u64,
}

impl Task {
  fn delay_blocking(&self) {
    println!("{}... [blocking]", self.afore);
    standard_sleep(Duration::from_secs(self.delay));
    println!("{}.", self.after);
  }

  async fn delay_async(&self) {
    println!("{}... [async]", self.afore);
    tokio_sleep(Duration::from_secs(self.delay)).await;
    println!("{}.", self.after);
  }
}

#[tokio::main(
  flavor = "multi_thread",
  worker_threads = 1
)]
async fn main() {
  let start_time: Instant = Instant::now();

  let chef_one: tokio::task::JoinHandle<()> = tokio::task::spawn(async {
    make_breakfast().await;
  });

  let chef_two: tokio::task::JoinHandle<()> = tokio::task::spawn(async {
    make_breakfast().await;
  });

  let _result: (
    Result<(), tokio::task::JoinError>,
    Result<(), tokio::task::JoinError>,
  ) = tokio::join!(chef_one, chef_two);

  let elapsed_time: Duration = start_time.elapsed();

  println!("It took {} seconds.", elapsed_time.as_secs());
}

async fn make_breakfast() {
  let coffee_mug_step = prep_coffee_mug();

  let coffee_step = make_coffee();

  let toast_step = make_toast();

  tokio::join!(coffee_mug_step, coffee_step, toast_step);
}

async fn make_coffee() {
  Task {
    afore: "Boiling kettle",
    after: "Kettle boiled",
    delay: 10,
  }
  .delay_async()
  .await;

  Task {
    afore: "Pouring boiled water",
    after: "Boiled water poured",
    delay: 3,
  }
  .delay_blocking();
}

async fn make_toast() {
  Task {
    afore: "Putting bread in toaster",
    after: "Bread toasted",
    delay: 10,
  }
  .delay_async()
  .await;

  Task {
    afore: "Buttering toasted bread",
    after: "Toasted bread buttered",
    delay: 5,
  }
  .delay_blocking();
}

async fn prep_coffee_mug() {
  tokio_sleep(Duration::from_secs(0)).await;

  Task {
    afore: "Pouring milk",
    after: "Milk poured",
    delay: 3,
  }
  .delay_blocking();

  Task {
    afore: "Adding instant coffee",
    after: "Instant coffee added",
    delay: 3,
  }
  .delay_blocking();
}
