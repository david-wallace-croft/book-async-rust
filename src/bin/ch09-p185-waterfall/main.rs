use ::std::error::Error;

type WaterfallResult = Result<String, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let output1: String = task1().await?;

  let output2: String = task2(output1).await?;

  let output3: String = task3(output2).await?;

  println!("{output3}");

  Ok(())
}

async fn task1() -> WaterfallResult {
  Ok("Task 1 completed".into())
}

async fn task2(input: String) -> WaterfallResult {
  Ok(format!("{input} then Task 2 completed"))
}

async fn task3(input: String) -> WaterfallResult {
  Ok(format!("{input} and finally Task 3 completed"))
}
