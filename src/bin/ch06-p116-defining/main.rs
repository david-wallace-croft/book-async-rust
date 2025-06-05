use self::display_future::DisplayFuture;
use self::heat_loss_future::HeatLossFuture;
use self::heater_future::HeaterFuture;
use ::tokio::task::JoinHandle;

mod display_future;
mod heat_loss_future;
mod heater_future;
mod statics;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
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
