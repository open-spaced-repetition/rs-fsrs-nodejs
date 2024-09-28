#![deny(clippy::all)]
// https://github.com/rust-lang/rust-analyzer/issues/17429
use chrono::DateTime;
use chrono::Utc;
use napi_derive::napi;
#[napi(js_name = "FSRS")]
#[derive(Debug)]
pub struct FSRS(fsrs::FSRS);
#[napi]
#[derive(Debug)]

pub struct ScheduledCards(fsrs::ScheduledCards);

#[napi]
impl ScheduledCards {
  #[napi]
  pub fn select_card(&self, rating: Rating) -> Card {
    Card(self.0.select_card(rating.into()))
  }
}

#[napi]
pub enum Rating {
  Again = 1,
  Hard = 2,
  Good = 3,
  Easy = 4,
}

impl From<Rating> for fsrs::Rating {
  fn from(value: Rating) -> Self {
    use fsrs::Rating as r;
    use Rating::*;
    match value {
      Again => r::Again,
      Hard => r::Hard,
      Good => r::Good,
      Easy => r::Easy,
    }
  }
}
impl From<fsrs::Rating> for Rating {
  fn from(value: fsrs::Rating) -> Self {
    use fsrs::Rating::*;
    use Rating as r;
    match value {
      Again => r::Again,
      Hard => r::Hard,
      Good => r::Good,
      Easy => r::Easy,
    }
  }
}

#[napi]
#[derive(Debug)]
pub struct Card(fsrs::Card);

#[napi]
#[derive(Debug)]
pub struct ReviewLog(fsrs::ReviewLog);

#[napi]
impl FSRS {
  #[napi(constructor)]
  pub fn new() -> Self {
    FSRS(fsrs::FSRS::default())
  }
  #[napi]
  pub fn schedule(&self, card: &Card, now: DateTime<Utc>) -> ScheduledCards {
    ScheduledCards(self.0.schedule(card.0.clone(), now))
  }
}

#[napi]
impl Card {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(fsrs::Card::new())
  }
  #[napi]
  pub fn log(&self) -> ReviewLog {
    ReviewLog(self.0.log.clone().unwrap())
  }
}
#[napi]
impl ReviewLog {
  #[napi]
  pub fn to_string(&self) -> String {
    format!("{:?}", self.0)
  }
}
