use ::std::thread;
use ::std::time::Duration;

mod async_mod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Hello, World!");

  let id: String = async_mod::send_add(1, 2)?;

  println!("id: {id}");

  thread::sleep(Duration::from_secs(4));

  println!("main sleep done");

  let result: i32 = async_mod::get_add(id)?;

  println!("result: {result}");

  Ok(())
}
