mod grid;

use grid::Grid;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut grid = Grid::new(&input);
  print!("Calculating the shortest path... ");
  grid.discover();
  println!("Got it! Just {} steps to go.", grid.get_distance());
  println!(
    "Wait I can start at multiple locations? Then it's just {} steps.",
    grid.get_distance_multi_start()
  )
}
