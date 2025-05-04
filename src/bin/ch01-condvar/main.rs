use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::{Arc, Condvar, Mutex, MutexGuard};
use ::std::thread::{self, JoinHandle};
use ::std::time::Duration;

fn main() {
  let shared_data: Arc<(Mutex<bool>, Condvar)> =
    Arc::new((Mutex::new(false), Condvar::new()));

  let shared_data_clone: Arc<(Mutex<bool>, Condvar)> = Arc::clone(&shared_data);

  let stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

  let stop_clone: Arc<AtomicBool> = Arc::clone(&stop);

  let _background_thread: JoinHandle<()> = thread::spawn(move || {
    let (mutex, cvar) = &*shared_data_clone;

    let mut received_value: MutexGuard<'_, bool> = mutex.lock().unwrap();

    while !stop.load(Relaxed) {
      received_value = cvar.wait(received_value).unwrap();

      println!("Received value: {}", *received_value);
    }
  });

  let updater_thread: JoinHandle<()> = thread::spawn(move || {
    let (mutex, cvar) = &*shared_data;

    let values: [bool; 4] = [
      false, true, false, true,
    ];

    for i in 0..4 {
      let update_value: bool = values[i as usize];

      println!("Updating value to {update_value}...");

      *mutex.lock().unwrap() = update_value;

      cvar.notify_one();

      thread::sleep(Duration::from_secs(4));
    }

    stop_clone.store(true, Relaxed);

    println!("STOP has been updated");

    cvar.notify_one();
  });

  updater_thread.join().unwrap();
}
