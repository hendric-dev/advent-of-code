mod grid;
mod movements;

use grid::Grid;
use movements::Movements;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let movements = Movements::new(&input);

  let mut grid = Grid::new(2);
  movements.0.iter().for_each(|movement| grid.move_head(movement));
  println!(
    "On a rope with 2 knots, the tail has visited {:#?} points.",
    grid.visited_points()
  );

  grid = Grid::new(10);
  movements.0.iter().for_each(|movement| grid.move_head(movement));
  println!(
    "On a rope with 10 knots, the tail has visited {:#?} points.",
    grid.visited_points()
  );
}
