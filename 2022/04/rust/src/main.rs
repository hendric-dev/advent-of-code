mod elf;
mod pair;

use pair::Pair;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let pairs: Vec<Pair> = input.split("\n").map(|row| Pair::from_input(row)).collect();
  println!(
    "Number of elves that to useless cleaning: {}",
    pairs.iter().filter(|pair| pair.one_fully_contained()).count()
  );
  println!(
    "Number of pairs that overlap some sections: {}",
    pairs.iter().filter(|pair| pair.overlaps()).count()
  );
}
