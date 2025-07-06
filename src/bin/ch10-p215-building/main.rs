use ::book_async_rust::ch10::{
  data::Data, executor::Executor, tcp_receiver::TcpReceiver,
  tcp_sender::TcpSender,
};
use std::{
  io,
  net::TcpStream,
  sync::{Arc, Mutex, mpsc::Receiver},
  thread,
  time::{Duration, Instant},
};

fn main() -> io::Result<()> {
  let mut executor: Executor = Default::default();

  let mut handles: Vec<Receiver<Result<String, io::Error>>> = Vec::new();

  let start = Instant::now();

  for i in 0..4_000 {
    let handle: Receiver<Result<String, io::Error>> =
      executor.spawn(send_data(i, i as u16, format!("Hello, server! {i}")));

    handles.push(handle);
  }

  thread::spawn(move || {
    loop {
      executor.poll();
    }
  });

  println!("Waiting for result...");

  for handle in handles {
    match handle.recv().unwrap() {
      Ok(result) => println!("Result: {result}"),
      Err(e) => println!("Error: {e}"),
    };
  }

  let duration: Duration = start.elapsed();

  println!("time elapsed in expensive_function() is {duration:?}");

  Ok(())
}

async fn send_data(
  field1: u32,
  field2: u16,
  field3: String,
) -> io::Result<String> {
  let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:7878")?));

  let message = Data {
    field1,
    field2,
    field3,
  };

  TcpSender {
    stream: stream.clone(),
    buffer: message.serialize()?,
  }
  .await?;

  let receiver = TcpReceiver {
    stream: stream.clone(),
    buffer: Vec::new(),
  };

  String::from_utf8(receiver.await?)
    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
}
