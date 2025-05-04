use ::std::io::Error;
use ::std::process::Output;
use ::tokio::process::Command;
use ::tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
  let mut handles: Vec<JoinHandle<Result<i32, Error>>> = vec![];

  for _ in 0..4 {
    let handle: JoinHandle<Result<i32, Error>> = tokio::spawn(async {
      let output: Result<Output, Error> =
        Command::new("./ch01-connection.exe").output().await;

      match output {
        Ok(output) => {
          println!(
            "Process completed with output: {}",
            String::from_utf8_lossy(&output.stdout)
          );

          Ok(output.status.code().unwrap_or(-1))
        },
        Err(e) => {
          eprintln!("Failed to run process: {}", e);

          Err(e)
        },
      }
    });

    handles.push(handle);
  }

  let mut results: Vec<Result<i32, Error>> = Vec::with_capacity(handles.len());

  for handle in handles {
    results.push(handle.await.unwrap());
  }

  for (i, result) in results.into_iter().enumerate() {
    match result {
      Ok(exit_code) => {
        println!("Process {} exited with code {}", i + 1, exit_code)
      },
      Err(e) => eprintln!("Process {} failed: {}", i + 1, e),
    }
  }
}
