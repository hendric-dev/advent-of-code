use crate::draw::Draw;

pub struct Round {
  pub opponent_draw: Draw,
  pub own_draw: Draw,
}

impl Round {
  pub fn adjust_for_draw(&mut self) {
    self.own_draw = self.opponent_draw.clone();
  }

  pub fn adjust_for_loss(&mut self) {
    match self.opponent_draw {
      Draw::Rock => self.own_draw = Draw::Scissors,
      Draw::Paper => self.own_draw = Draw::Rock,
      Draw::Scissors => self.own_draw = Draw::Paper,
    }
  }

  pub fn adjust_for_win(&mut self) {
    match self.opponent_draw {
      Draw::Rock => self.own_draw = Draw::Paper,
      Draw::Paper => self.own_draw = Draw::Scissors,
      Draw::Scissors => self.own_draw = Draw::Rock,
    }
  }

  pub fn score(&self) -> i32 {
    let mut score = 0;
    match self.own_draw {
      Draw::Rock => score += 1,
      Draw::Paper => score += 2,
      Draw::Scissors => score += 3,
    }
    match self.outcome() {
      Outcome::Win => score += 6,
      Outcome::Draw => score += 3,
      Outcome::Loss => score += 0,
    }
    score
  }

  fn outcome(&self) -> Outcome {
    match self.opponent_draw {
      Draw::Rock => match self.own_draw {
        Draw::Rock => Outcome::Draw,
        Draw::Paper => Outcome::Win,
        Draw::Scissors => Outcome::Loss,
      },
      Draw::Paper => match self.own_draw {
        Draw::Rock => Outcome::Loss,
        Draw::Paper => Outcome::Draw,
        Draw::Scissors => Outcome::Win,
      },
      Draw::Scissors => match self.own_draw {
        Draw::Rock => Outcome::Win,
        Draw::Paper => Outcome::Loss,
        Draw::Scissors => Outcome::Draw,
      },
    }
  }
}

enum Outcome {
  Draw,
  Loss,
  Win,
}
