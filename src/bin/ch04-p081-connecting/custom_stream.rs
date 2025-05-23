#![expect(dead_code)]

use ::async_native_tls::TlsStream;
use ::smol::Async;
use ::smol::prelude::{AsyncRead, AsyncWrite};
use ::std::io::Error;
use ::std::net::Shutdown;
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
  ) -> Poll<std::io::Result<()>> {
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

impl tokio::io::AsyncWrite for CustomStream {
  fn poll_write(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buf: &[u8],
  ) -> Poll<Result<usize, Error>> {
    match &mut *self {
      CustomStream::Plain(s) => Pin::new(s).poll_write(context, buf),
      CustomStream::Tls(s) => Pin::new(s).poll_write(context, buf),
    }
  }

  fn poll_flush(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Result<(), Error>> {
    match &mut *self {
      CustomStream::Plain(s) => Pin::new(s).poll_flush(context),
      CustomStream::Tls(s) => Pin::new(s).poll_flush(context),
    }
  }

  fn poll_shutdown(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Result<(), Error>> {
    match &mut *self {
      CustomStream::Plain(s) => {
        s.get_ref().shutdown(Shutdown::Write)?;

        Poll::Ready(Ok(()))
      },
      CustomStream::Tls(s) => Pin::new(s).poll_close(context),
    }
  }
}

impl hyper::client::connect::Connection for CustomStream {
  fn connected(&self) -> hyper::client::connect::Connected {
    hyper::client::connect::Connected::new()
  }
}
