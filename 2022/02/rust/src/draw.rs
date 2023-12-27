#[derive(Clone)]
pub enum Draw {
  Rock,
  Paper,
  Scissors,
}

pub fn from_input(input: &str) -> Draw {
  match input {
    "A" => Draw::Rock,
    "B" => Draw::Paper,
    "C" => Draw::Scissors,
    "X" => Draw::Rock,
    "Y" => Draw::Paper,
    "Z" => Draw::Scissors,
    _ => panic!("{input} is an invalid value. Use A, B, C, X, Y or Z instead."),
  }
}
