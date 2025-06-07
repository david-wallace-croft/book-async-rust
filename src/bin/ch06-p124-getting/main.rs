#![feature(coroutine_trait)]
#![feature(coroutines)]

use self::{
  display_future::DisplayFuture,
  heat_loss_future::HeatLossFuture,
  heater_future::HeaterFuture,
  statics::{DESIRED_TEMP, DEVICE_STATE, HEAT_ON, INPUT, TEMP},
};
use ::device_query::{DeviceEvents, Keycode};
use ::std::{
  io::{self, Stdout, StdoutLock, Write},
  sync::{MutexGuard, atomic::Ordering},
};
use ::tokio::task::JoinHandle;

mod display_future;
mod heat_loss_future;
mod heater_future;
mod statics;

#[tokio::main]
async fn main() {
  let _guard = DEVICE_STATE.on_key_down(|key: &Keycode| {
    let mut input: MutexGuard<'_, String> = INPUT.lock().unwrap();

    input.push_str(&key.to_string());

    std::mem::drop(input);

    render(
      TEMP.load(Ordering::SeqCst),
      DESIRED_TEMP.load(Ordering::SeqCst),
      HEAT_ON.load(Ordering::SeqCst),
      INPUT.lock().unwrap().clone(),
    );
  });

  let display: JoinHandle<()> = tokio::spawn(async {
    DisplayFuture::default().await;
  });

  let heat_loss: JoinHandle<()> = tokio::spawn(async {
    HeatLossFuture::default().await;
  });

  let heater: JoinHandle<()> = tokio::spawn(async {
    HeaterFuture::default().await;
  });

  display.await.unwrap();

  heat_loss.await.unwrap();

  heater.await.unwrap();
}

fn render(
  temp: i16,
  desired_temp: i16,
  heat_on: bool,
  input: String,
) {
  clearscreen::clear().unwrap();

  let stdout: Stdout = io::stdout();

  let mut handle: StdoutLock<'static> = stdout.lock();

  println!(
    "Temperature: {}\nDesired Temp: {}\nHeater On: {}",
    temp as f32 / 100.,
    desired_temp as f32 / 100.,
    heat_on
  );

  print!("Input: {}", input);

  handle.flush().unwrap();
}
