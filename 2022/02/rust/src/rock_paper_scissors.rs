use crate::{draw, draw::Draw, round::Round};

pub struct RockPaperScissors(Vec<Round>);

impl RockPaperScissors {
  pub fn from_input(input: &str) -> Self {
    Self(
      input
        .split("\n")
        .map(|draw| {
          let (opponent, own) = draw.split_once(" ").expect("Failed to split input at space");
          Round {
            opponent_draw: draw::from_input(opponent),
            own_draw: draw::from_input(own),
          }
        })
        .collect::<Vec<Round>>(),
    )
  }

  pub fn modify_with_strategy_guide(&mut self) {
    self.0.iter_mut().for_each(|round| match round.own_draw {
      Draw::Rock => round.adjust_for_loss(),
      Draw::Paper => round.adjust_for_draw(),
      Draw::Scissors => round.adjust_for_win(),
    })
  }

  pub fn score(&self) -> i32 {
    self.0.iter().map(|round| round.score()).sum()
  }
}
