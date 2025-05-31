use ::std::borrow::Cow;
use ::std::process::{Command, Output};

fn main() {
  let output: Output = Command::new("./ch01-connection")
    .output()
    .expect("Failed to execute command");

  if output.status.success() {
    let stdout: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

    println!("Output: {stdout}");
  } else {
    let stderr: Cow<'_, str> = String::from_utf8_lossy(&output.stderr);

    eprintln!("Error: {stderr}");
  }
}
