#![expect(unused_imports)]

use ::core::sync::atomic::Ordering;
use ::std::{
  future::Future,
  pin::Pin,
  sync::{
    Arc, LazyLock,
    atomic::{AtomicBool, AtomicI16},
  },
  task::{Context, Poll},
  time::{Duration, Instant},
};

mod display_future;
mod heater_future;
mod statics;

#[cfg(test)]
mod tests;

fn main() {
  todo!()
}
