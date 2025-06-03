use super::statics::{DESIRED_TEMP, HEAT_ON, TEMP};
use ::std::{
  pin::Pin,
  sync::atomic::Ordering,
  task::{Context, Poll},
};

pub struct DisplayFuture {
  pub temp_snapshot: i16,
}

impl Default for DisplayFuture {
  fn default() -> Self {
    Self {
      temp_snapshot: TEMP.load(Ordering::SeqCst),
    }
  }
}

impl Future for DisplayFuture {
  type Output = ();

  fn poll(
    mut self: Pin<&mut Self>,
    context: &mut Context<'_>,
  ) -> Poll<Self::Output> {
    let current_snapshot: i16 = TEMP.load(Ordering::SeqCst);

    let desired_temp: i16 = DESIRED_TEMP.load(Ordering::SeqCst);

    let heat_on: bool = HEAT_ON.load(Ordering::SeqCst);

    if current_snapshot == self.temp_snapshot {
      context.waker().wake_by_ref();

      return Poll::Pending;
    }

    if current_snapshot < desired_temp && !heat_on {
      HEAT_ON.store(true, Ordering::SeqCst);
    } else if current_snapshot > desired_temp && heat_on {
      HEAT_ON.store(false, Ordering::SeqCst);
    }

    clearscreen::clear().unwrap();

    println!(
      "Temperature: {}\nDesired Temp: {}\nHeater On: {}",
      current_snapshot as f32 / 100.,
      desired_temp as f32 / 100.,
      heat_on
    );

    self.temp_snapshot = current_snapshot;

    context.waker().wake_by_ref();

    Poll::Pending
  }
}
