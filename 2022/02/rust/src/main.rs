mod draw;
mod rock_paper_scissors;
mod round;

use rock_paper_scissors::RockPaperScissors;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut game = RockPaperScissors::from_input(&input);
  println!("Score according to naive idea of stretegy guide: {}", game.score());
  game.modify_with_strategy_guide();
  println!("Score according to real stretegy guide: {}", game.score());
}
