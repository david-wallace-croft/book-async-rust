use ::book_async_rust::ch01;

fn main() {
  // ch01::fibonacci::run();

  let _result: Result<(), reqwest::Error> = ch01::http::run();
}
