use ::async_task::Task;
use ::futures_lite::future;
use ::hyper::client::HttpConnector;
use ::hyper::{Body, Client, Request, Response, Uri};

mod custom_executor;
mod future_type;
#[macro_use]
mod queue;

fn main() {
  let url: &'static str = "http://www.rust-lang.org";

  let uri: Uri = url.parse().unwrap();

  let request: Request<Body> = Request::builder()
    .method("GET")
    .uri(uri)
    .header("User-Agent", "hyper/0.14.2")
    .header("Accept", "text/html")
    .body(Body::empty())
    .unwrap();

  let future = async {
    let client: Client<HttpConnector> = Client::new();

    client.request(request).await.unwrap()
  };

  let test: Task<Response<Body>> = spawn_task!(future);

  let response: Response<Body> = future::block_on(test);

  println!("Response status: {}", response.status());
}
