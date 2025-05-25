use super::constants::SERVER;
use ::mio::net::TcpListener;
use ::mio::{Events, Poll as MioPoll};
use ::std::io::Read;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::std::time::Duration;

pub struct ServerFuture {
  pub server: TcpListener,
  pub mio_poll: MioPoll,
}

impl Future for ServerFuture {
  type Output = String;

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    let mut events = Events::with_capacity(1);

    self
      .mio_poll
      .poll(&mut events, Some(Duration::from_millis(200)))
      .unwrap();

    for event in events.iter() {
      if event.token() == SERVER && event.is_readable() {
        let (mut stream, _) = self.server.accept().unwrap();

        let mut buffer = [0u8; 1024];

        let mut received_data = Vec::new();

        loop {
          match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
              received_data.extend_from_slice(&buffer[..n]);
            },
            Ok(_) => {
              break;
            },
            Err(e) => {
              eprintln!("Error reading from stream: {}", e);

              break;
            },
          }
        }

        if !received_data.is_empty() {
          let received_str = String::from_utf8_lossy(&received_data);

          return Poll::Ready(received_str.to_string());
        }

        context.waker().wake_by_ref();

        return Poll::Pending;
      }
    }

    context.waker().wake_by_ref();

    Poll::Pending
  }
}
