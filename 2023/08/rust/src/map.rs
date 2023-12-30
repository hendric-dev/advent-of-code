use std::{collections::BTreeMap, path};

#[derive(Debug)]
pub struct Map {
  pub instructions: Vec<String>,
  pub network: BTreeMap<String, Turnoff>,
}

#[derive(Debug)]
pub struct Turnoff {
  pub left: String,
  pub right: String,
}

impl Map {
  pub fn from_input(input: &str) -> Self {
    Self {
      instructions: Self::parse_instructions(input),
      network: Self::parse_network(input),
    }
  }

  pub fn travel(&self, start: &str, destination: &str) -> usize {
    let mut steps = 0;
    let mut current = start;

    while !current.ends_with(destination) {
      let instruction = &self.instructions[steps % self.instructions.len()];
      steps += 1;
      match instruction.as_ref() {
        "L" => current = &self.network.get(current).unwrap().left,
        "R" => current = &self.network.get(current).unwrap().right,
        _ => panic!("Unknown instruction: {}", instruction),
      }
    }

    steps
  }

  pub fn travel_in_ghost_mode(&self, start: &str, destination: &str) -> usize {
    let starting_nodes: Vec<&str> = self
      .network
      .keys()
      .filter(|key| key.ends_with(start))
      .map(|key| key.as_ref())
      .collect();

    let mut steps: Vec<usize> = vec![];
    for starting_node in starting_nodes {
      steps.push(self.travel(starting_node, destination));
    }

    steps.iter().fold(1, |acc, &step| lcm(acc, step))
  }

  fn parse_instructions(input: &str) -> Vec<String> {
    input
      .lines()
      .next()
      .expect("Failed to read instructions from input")
      .split("")
      .filter(|instruction| !instruction.is_empty())
      .map(|instruction| String::from(instruction))
      .collect()
  }

  fn parse_network(input: &str) -> BTreeMap<String, Turnoff> {
    let mut network: BTreeMap<String, Turnoff> = BTreeMap::new();
    input.lines().filter(|line| !line.is_empty()).skip(1).for_each(|line| {
      let modified_line = line
        .replace("=", ",")
        .replace("(", "")
        .replace(")", "")
        .replace(" ", "");
      let mut split = modified_line.split(",");
      let start = split.next().expect("Failed to get start of network definition");
      let left = split.next().expect("Failed to get left turnoff of network definition");
      let right = split.next().expect("Failed to get right turnoff of network definition");

      network.insert(
        String::from(start),
        Turnoff {
          left: String::from(left),
          right: String::from(right),
        },
      );
    });

    network
  }
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn lcm(a: usize, b: usize) -> usize {
  a / gcd(a, b) * b
}
