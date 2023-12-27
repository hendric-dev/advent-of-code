mod instructions;
mod stacks;

use instructions::Instructions;
use stacks::Stacks;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut split = input.split("\n\n");
  let mut stacks_part_one = Stacks::from_input(split.next().expect("Failed to get stacks from input"));
  let mut stacks_part_two = stacks_part_one.clone();
  let instructions = Instructions::from_input(split.next().expect("Failed to get instructions from input"));
  instructions.0.iter().for_each(|instruction| {
    stacks_part_one.apply_instruction_single_crate(instruction);
    stacks_part_two.apply_instruction_multi_crate(instruction);
  });
  println!(
    "The top crates when using the CrateMover9000 are named {}",
    stacks_part_one.get_top_crate_names()
  );
  println!(
    "The top crates when using the CrateMover9001 are named {}",
    stacks_part_two.get_top_crate_names()
  );
}
