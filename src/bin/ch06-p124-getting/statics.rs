use ::device_query::DeviceState;
use ::std::sync::Arc;
use ::std::sync::LazyLock;
use ::std::sync::Mutex;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::AtomicI16;

pub static DESIRED_TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2100)));

pub static DEVICE_STATE: LazyLock<Arc<DeviceState>> =
  LazyLock::new(|| Arc::new(DeviceState::new()));

pub static HEAT_ON: LazyLock<Arc<AtomicBool>> =
  LazyLock::new(|| Arc::new(AtomicBool::new(false)));

pub static INPUT: LazyLock<Arc<Mutex<String>>> =
  LazyLock::new(|| Arc::new(Mutex::new(String::new())));

pub static TEMP: LazyLock<Arc<AtomicI16>> =
  LazyLock::new(|| Arc::new(AtomicI16::new(2090)));
