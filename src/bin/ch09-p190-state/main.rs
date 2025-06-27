use self::event::Event;
use self::state::State;

mod event;
mod state;

#[tokio::main]
async fn main() {
  let mut state = State::On;

  state = state.transition(Event::SwitchOff).await;
  state = state.transition(Event::SwitchOn).await;
  state = state.transition(Event::SwitchOn).await;

  match state {
    State::On => println!("State machine is in the On state"),
    _ => println!("State machine is not in the expected state"),
  }
}
