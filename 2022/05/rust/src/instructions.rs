#[derive(Debug)]
pub struct Instructions(pub Vec<Instruction>);

impl Instructions {
  pub fn from_input(input: &str) -> Self {
    Self(input.split("\n").map(|row| Instruction::from_input(row)).collect())
  }
}

#[derive(Debug)]
pub struct Instruction {
  pub from: usize,
  pub to: usize,
  pub times: usize,
}

impl Instruction {
  fn from_input(input: &str) -> Self {
    let mut instruction = input
      .split(" ")
      .filter(|part| str::parse::<usize>(part).is_ok())
      .map(|part| str::parse::<usize>(part).unwrap());

    let times = instruction.next().unwrap();
    let from = instruction.next().unwrap();
    let to = instruction.next().unwrap();

    Self { from, to, times }
  }
}
