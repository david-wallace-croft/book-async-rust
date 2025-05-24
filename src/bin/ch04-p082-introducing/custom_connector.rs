use super::custom_stream::CustomStream;
use ::anyhow::Error;
use ::anyhow::{Context as anyhowContext, bail};
use ::hyper::Uri;
use ::hyper::service::Service;
use ::smol::Async;
use ::std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use ::std::pin::Pin;
use ::std::task::{Context, Poll};

#[derive(Clone)]
pub struct CustomConnector;

impl Service<Uri> for CustomConnector {
  type Response = CustomStream;

  type Error = Error;

  type Future =
    Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(
    &mut self,
    _context: &mut Context<'_>,
  ) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(
    &mut self,
    uri: Uri,
  ) -> Self::Future {
    Box::pin(async move {
      let host: Result<&str, Error> = uri.host().context("cannot parse host");

      match uri.scheme_str() {
        Some("http") => {
          todo!()
        },
        Some("https") => {
          let host_0: String = host.unwrap().to_string();

          let host_1: String = host_0.clone();

          let socket_addr: SocketAddr = {
            let port: u16 = uri.port_u16().unwrap_or(443);

            smol::unblock(move || (host_1.clone(), port).to_socket_addrs())
              .await?
              .next()
              .context("cannot resolve address")?
          };

          let stream: Async<TcpStream> =
            Async::<TcpStream>::connect(socket_addr).await?;

          let stream: async_native_tls::TlsStream<Async<TcpStream>> =
            async_native_tls::connect(host_0, stream).await?;

          Ok(CustomStream::Tls(stream))
        },
        scheme => bail!("unsupported scheme: {:?}", scheme),
      }
    })
  }
}
