use ::std::sync::Arc;
use ::std::sync::LazyLock;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicI16;

pub static DESIRED_TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2100)));

pub static HEAT_ON: LazyLock<Arc<AtomicBool>> =
  LazyLock::new(|| Arc::new(AtomicBool::new(false)));

pub static TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2090)));
