use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut elves = Elves::from_input(&input);

  println!("The biggest elf carries {} calories.", elves.max_calories(1));
  println!("The three biggest elves carry {:?} calories.", elves.max_calories(3));
}

#[derive(Debug)]
struct Elf {
  calories: Vec<i32>,
}

impl Elf {
  fn from_calories_string(calories: &str) -> Self {
    Self {
      calories: calories
        .split("\n")
        .map(|value| value.parse().expect("Failed to parse input number"))
        .collect(),
    }
  }

  fn total_calories(&self) -> i32 {
    self.calories.iter().sum()
  }
}

struct Elves(Vec<Elf>);

impl Elves {
  fn from_input(input: &str) -> Self {
    Self(
      input
        .split("\n\n")
        .map(|calories| Elf::from_calories_string(calories))
        .collect::<Vec<Elf>>(),
    )
  }

  fn max_calories(&mut self, number_of_elves: usize) -> i32 {
    self.0.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    self
      .0
      .iter()
      .take(number_of_elves)
      .map(|elf| elf.total_calories())
      .sum::<i32>()
  }
}
