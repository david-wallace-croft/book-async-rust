#![allow(clippy::vec_init_then_push)]

use self::custom_connector::CustomConnector;
use self::custom_executor::CustomExecutor;
use self::runtime::Runtime;
use ::futures_lite::future;
use ::hyper::{Body, Client, Request, Response};

mod custom_connector;
mod custom_executor;
mod custom_stream;
mod future_type;
#[macro_use]
mod queue;
mod runtime;

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

fn main() {
  Runtime::new().with_low_num(2).with_high_num(4).run();

  let future = async {
    let req: Request<Body> = Request::get("https://www.rust-lang.org")
      .body(Body::empty())
      .unwrap();

    let response: Response<Body> = fetch(req).await.unwrap();

    let body_bytes: hyper::body::Bytes =
      hyper::body::to_bytes(response.into_body()).await.unwrap();

    let html: String = String::from_utf8(body_bytes.to_vec()).unwrap();

    println!("{html}");
  };

  let test: smol::Task<()> = spawn_task!(future);

  future::block_on(test);
}

pub async fn fetch(req: Request<Body>) -> hyper::Result<Response<Body>> {
  let response: Response<Body> = Client::builder()
    .executor(CustomExecutor)
    .build::<_, Body>(CustomConnector)
    .request(req)
    .await?;

  Ok(response)
}
