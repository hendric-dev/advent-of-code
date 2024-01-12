mod pipe_grid;

use pipe_grid::PipeGrid;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut grid = PipeGrid::from(&input);
  println!("[Part 1] The longest distance in the loop is {}", grid.measure_loop());
  println!(
    "[Part 2] The amount of enclosed tiles is {}",
    grid.count_enclosed_tiles()
  );
}
