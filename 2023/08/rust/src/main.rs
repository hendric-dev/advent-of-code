mod map;

use map::Map;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let map = Map::from_input(&input);
  println!("[Part 1] {} steps are needed to reach 'ZZZ'", map.travel("AAA", "ZZZ"));
  println!(
    "[Part 2] Ghost mode activated... {} steps are needed to reach all destinations",
    map.travel_in_ghost_mode("A", "Z")
  );
}
