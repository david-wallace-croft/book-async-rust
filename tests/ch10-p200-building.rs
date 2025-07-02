use ::book_async_rust::ch10::waker;
use ::std::task::RawWaker;

#[test]
fn test_create_raw_waker() {
  let _raw_waker: RawWaker = waker::create_raw_waker();
}
