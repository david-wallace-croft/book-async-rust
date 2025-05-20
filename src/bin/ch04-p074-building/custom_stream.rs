#![expect(dead_code)]

use ::async_native_tls::TlsStream;
use ::smol::Async;
use ::std::net::TcpStream;

pub enum CustomStream {
  Plain(Async<TcpStream>),
  Tls(TlsStream<Async<TcpStream>>),
}
