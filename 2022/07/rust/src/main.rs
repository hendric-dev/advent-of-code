mod commands;
mod directories;

use commands::Commands;
use directories::Directories;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let commands = Commands::new(&input);
  let mut directories = Directories::new();

  commands
    .0
    .iter()
    .for_each(|command| directories.execute_command(command));

  let used_space: i32 = directories.used_space();
  let free_space = 70000000 - used_space;
  let missing_space = 30000000 - free_space;

  println!(
    "The sum of all small directories is {}",
    directories.sum_of_directories_smaller_than(&100000),
  );

  println!(
    "To free enough space, the directory of size {} should be removed",
    directories.size_of_smallest_directory_above(&missing_space),
  )
}
