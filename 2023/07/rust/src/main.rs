mod hand;

use hand::Hands;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut hands = Hands::from_input(&input, hand::Mode::Normal);
  hands.sort_by_strength();

  println!("[Part 1] The total winnings are: {}", hands.get_winnings());

  hands = Hands::from_input(&input, hand::Mode::Joker);
  hands.sort_by_strength();
  println!("Hands: {:#?}", hands);

  println!("[Part 2] The total winnings are: {}", hands.get_winnings());
}
