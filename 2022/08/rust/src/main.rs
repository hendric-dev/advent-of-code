mod grid;

use grid::Grid;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let grid = Grid::new(&input);

  println!("There are {} trees visible from the outside", grid.visible_trees());
  println!("The best spot has a scenic score of {}", grid.highest_scenic_score());
}
