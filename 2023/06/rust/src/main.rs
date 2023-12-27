mod race;

use race::Races;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut races = Races::from_input(&input);

  println!(
    "[Part 1] Multiplied number of ways to beat the record: {}",
    races.multiply_record_breaking_button_presses()
  );

  races = Races::from_input_as_single_race(&input);

  println!(
    "[Part 2] Multiplied number of ways to beat the record: {}",
    races.multiply_record_breaking_button_presses()
  );
}
