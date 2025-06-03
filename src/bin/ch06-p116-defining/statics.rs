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

pub static DESIRED_TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2100)));

pub static HEAT_ON: LazyLock<Arc<AtomicBool>> =
  LazyLock::new(|| Arc::new(AtomicBool::new(false)));

pub static TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2090)));
