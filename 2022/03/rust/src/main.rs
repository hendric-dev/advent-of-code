mod groups;
mod item;
mod rucksack;

use groups::Groups;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let groups = Groups::from_input(&input, 3);
  println!(
    "The priorities of duplicate items in each compartment is {}",
    groups.duplicates_priority_sum()
  );
  println!("The priorities of all badges is {}", groups.badge_priority_sum());
}
