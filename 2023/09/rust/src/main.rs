mod dataset;

use dataset::Dataset;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut dataset = Dataset::from_input(&input);
  dataset.calculate_sequences();

  println!(
    "[Part 1] Extrapolating data at the end... complete: {}",
    dataset.extrapolate("end").iter().sum::<i32>()
  );
  println!(
    "[Part 2] Extrapolating data at the front... complete: {}",
    dataset.extrapolate("front").iter().sum::<i32>()
  );
}
