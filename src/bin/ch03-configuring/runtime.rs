use super::future_type::FutureType;
use crate::{join, spawn_task};
use ::async_task::Task;
use ::std::{env, thread};

// use super::spawn_task;

pub struct Runtime {
  high_num: usize,
  low_num: usize,
}

impl Runtime {
  pub fn new() -> Self {
    let num_cores = thread::available_parallelism().unwrap().get();

    Self {
      // TODO: Why does the book subtract 2 instead of 1?
      high_num: num_cores.saturating_sub(1),
      low_num: 1,
    }
  }

  pub fn with_high_num(
    mut self,
    num: usize,
  ) -> Self {
    self.high_num = num;

    self
  }

  pub fn with_low_num(
    mut self,
    num: usize,
  ) -> Self {
    self.low_num = num;

    self
  }

  pub fn run(&self) {
    unsafe {
      env::set_var("HIGH_NUM", self.high_num.to_string());

      env::set_var("LOW_NUM", self.high_num.to_string());
    }

    let high: Task<()> = spawn_task!(async {}, FutureType::High);

    let low: Task<()> = spawn_task!(async {}, FutureType::Low);

    join!(high, low);
  }
}
