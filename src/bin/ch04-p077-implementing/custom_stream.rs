#![expect(dead_code)]

use ::async_native_tls::TlsStream;
use ::smol::Async;
use ::smol::io::Result;
use ::smol::prelude::*;
use ::std::net::TcpStream;
use ::std::pin::Pin;
use ::std::task::{Context, Poll};
use ::tokio::io::ReadBuf;

pub enum CustomStream {
  Plain(Async<TcpStream>),
  Tls(TlsStream<Async<TcpStream>>),
}

impl tokio::io::AsyncRead for CustomStream {
  fn poll_read(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buf: &mut ReadBuf<'_>,
  ) -> Poll<Result<()>> {
    match &mut *self {
      CustomStream::Plain(s) => Pin::new(s)
        .poll_read(context, buf.initialize_unfilled())
        .map_ok(|size| buf.advance(size)),
      CustomStream::Tls(s) => Pin::new(s)
        .poll_read(context, buf.initialize_unfilled())
        .map_ok(|size| buf.advance(size)),
    }
  }
}
