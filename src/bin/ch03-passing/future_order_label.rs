use super::future_type::FutureType;

pub trait FutureOrderLabel: Future {
  fn get_order(&self) -> FutureType;
}
