use std::sync::atomic::{AtomicI64, Ordering};

#[test]
fn test_0() {
  let some_var: AtomicI64 = AtomicI64::new(5);

  assert_eq!(
    some_var.compare_exchange(5, 10, Ordering::Acquire, Ordering::Relaxed),
    Ok(5),
  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);

  assert_eq!(
    some_var.compare_exchange(6, 12, Ordering::Acquire, Ordering::Relaxed),
    Err(10),
  );

  assert_eq!(some_var.load(Ordering::Relaxed), 10);
}
