use ::std::ptr;

struct SelfReferential {
  data: String,
  self_pointer: *const String,
}

impl SelfReferential {
  fn new(data: String) -> Self {
    let mut self_referential = Self {
      data,
      self_pointer: ptr::null(),
    };

    self_referential.self_pointer = &self_referential.data as *const String;

    self_referential
  }

  fn print(&self) {
    unsafe {
      println!("{}", *self.self_pointer);
    }
  }
}

fn main() {
  let first: SelfReferential = SelfReferential::new("first".into());

  let moved_first: SelfReferential = first;

  moved_first.print();
}
