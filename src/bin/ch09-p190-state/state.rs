use super::event::Event;

pub enum State {
  Off,
  On,
}

impl State {
  pub async fn transition(
    self,
    event: Event,
  ) -> Self {
    match (&self, event) {
      (State::On, Event::SwitchOff) => {
        println!("Transitioning to the Off state");

        State::Off
      },
      (State::Off, Event::SwitchOn) => {
        println!("Transitioning to the On state");

        State::On
      },
      _ => {
        println!("No transition possible");

        self
      },
    }
  }
}
