mod word_grid;

use std::fs::read_to_string;
use word_grid::WordGrid;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let word_grid = WordGrid::from_input(&input);
  let xmas_count = word_grid.count_xmas();
  let x_mas_count = word_grid.count_x_mas();

  println!("[Part 1] 🤖 Beep, Boop... counted {xmas_count} times the word XMAS");
  println!("[Part 2] 🤖 Beep, Boop... counted {x_mas_count} times the cross shaped X-MAS");
}
