#[cfg(test)]
mod tests {
  use ::mockito::{Matcher, Mock, ServerGuard};
  use ::reqwest::{Client, Response};
  use ::tokio::runtime::{Builder, Runtime};
  use ::tokio::task::JoinHandle;

  #[test]
  fn test_networking() {
    let mut server: ServerGuard = mockito::Server::new();

    let url: String = server.url();

    let mock: Mock = server
      .mock("GET", "/my-endpoint")
      .match_query(Matcher::AllOf(vec![
        Matcher::UrlEncoded("param1".into(), "value1".into()),
        Matcher::UrlEncoded("param2".into(), "value2".into()),
      ]))
      .with_status(201)
      .with_body("world")
      .expect(5)
      .create();

    let runtime: Runtime = Builder::new_current_thread()
      .enable_io()
      .enable_time()
      .build()
      .unwrap();

    let mut handles: Vec<JoinHandle<Response>> = vec![];

    for _ in 0..5 {
      let url_clone: String = url.clone();

      let join_handle: JoinHandle<Response> = runtime.spawn(async move {
        let client: Client = Client::new();

        let url: &String =
          &format!("{url_clone}/my-endpoint?param1=value1&param2=value2");

        client.get(url).send().await.unwrap()
      });

      handles.push(join_handle);
    }

    for handle in handles {
      let _ = runtime.block_on(handle).unwrap();
    }

    mock.assert();
  }
}
