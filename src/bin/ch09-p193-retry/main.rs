use ::std::error::Error;
use ::tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
  let outcome: Result<String, Box<dyn Error + 'static>> = do_something().await;

  println!("Outcome: {outcome:?}");
}

async fn do_something() -> Result<String, Box<dyn Error>> {
  let mut milliseconds: u64 = 1_000;

  let total_count: i32 = 5;

  let mut count: i32 = 0;

  let result: String;

  loop {
    match get_data().await {
      Ok(data) => {
        result = data;

        break;
      },
      Err(err) => {
        println!("Error: {err}");

        count += 1;

        if count == total_count {
          return Err(err);
        }
      },
    }

    time::sleep(Duration::from_millis(milliseconds)).await;

    milliseconds *= 2;
  }

  Ok(result)
}

async fn get_data() -> Result<String, Box<dyn Error>> {
  Err("Error".into())
}
