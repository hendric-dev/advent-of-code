mod monkeys;

use monkeys::Monkeys;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut monkeys = Monkeys::new(&input, true);
  print!("Calculating the monkey business after 20 rounds... ");
  monkeys.simulate_rounds(20);
  println!("Found it: {}", monkeys.monkey_business());

  monkeys = Monkeys::new(&input, false);
  print!("Calculating the monkey business after 10000 rounds while I'm a lot more worried... ");
  monkeys.simulate_rounds(10000);
  println!("Found it: {}", monkeys.monkey_business());
}
