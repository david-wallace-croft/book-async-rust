#![allow(clippy::vec_init_then_push)]

use self::constants::CLIENT;
use self::constants::SERVER;
use self::custom_connector::CustomConnector;
use self::custom_executor::CustomExecutor;
use self::runtime::Runtime;
use self::server_future::ServerFuture;
use ::futures_lite::future;
use ::hyper::{Body, Client, Request, Response};
use ::mio::net::{TcpListener, TcpStream};
use ::mio::{Events, Interest, Poll as MioPoll};
use ::std::error::Error;
use ::std::io::{Error as IoError, Write};
use ::std::net::SocketAddr;

mod constants;
mod custom_connector;
mod custom_executor;
mod custom_stream;
mod future_type;
#[macro_use]
mod queue;
mod runtime;
mod server_future;

#[macro_export]
macro_rules! join {

  ($($future:expr),*) => {
    {
      let mut results = Vec::new();

      $(
        results.push(::futures_lite::future::block_on($future));
      )*

      results
    }
  };
}

fn main() -> Result<(), Box<dyn Error>> {
  Runtime::new().with_low_num(2).with_high_num(4).run();

  let addr: SocketAddr = "127.0.0.1:13265".parse()?;

  let mut server: TcpListener = TcpListener::bind(addr)?;

  let mut stream: TcpStream = TcpStream::connect(server.local_addr()?)?;

  let mio_poll: MioPoll = MioPoll::new()?;

  mio_poll
    .registry()
    .register(&mut server, SERVER, Interest::READABLE)?;

  let server_worker: ServerFuture = ServerFuture {
    mio_poll,
    server,
  };

  let test: smol::Task<String> = spawn_task!(server_worker);

  let mut client_mio_poll: MioPoll = MioPoll::new()?;

  let _result: Result<(), IoError> = client_mio_poll.registry().register(
    &mut stream,
    CLIENT,
    Interest::WRITABLE,
  );

  let mut events: Events = Events::with_capacity(128);

  client_mio_poll.poll(&mut events, None).unwrap();

  for event in events.iter() {
    if event.token() == CLIENT && event.is_writable() {
      let message = "That's so dingo!\n";

      let _ = stream.write_all(message.as_bytes());
    }
  }

  let outcome: String = future::block_on(test);

  println!("outcome: {outcome}");

  Ok(())
}

pub async fn fetch(req: Request<Body>) -> hyper::Result<Response<Body>> {
  let client: Client<CustomConnector> = Client::builder()
    .executor(CustomExecutor)
    .build::<_, Body>(CustomConnector);

  let response_future: hyper::client::ResponseFuture = client.request(req);

  let response: Response<Body> = response_future.await?;

  Ok(response)
}
