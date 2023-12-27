use crate::instructions::Instruction;

#[derive(Clone, Debug)]
pub struct Stacks(Vec<Stack>);

impl Stacks {
  pub fn from_input(input: &str) -> Self {
    let mut rows: Vec<&str> = input.split("\n").collect();
    let length = str::parse::<usize>(rows.pop().unwrap().split(" ").last().unwrap()).unwrap();

    let mut stacks = Self(vec![]);
    for _ in 1..=length {
      stacks.0.push(Stack(vec![]))
    }

    rows.iter().rev().for_each(|row| {
      row
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk[1])
        .enumerate()
        .for_each(|(index, item)| {
          if item.to_string() != " " {
            stacks.add_crate(index, item.to_string().as_ref());
          }
        });
    });

    stacks
  }

  pub fn apply_instruction_single_crate(&mut self, instruction: &Instruction) {
    for _ in 1..=instruction.times {
      let item = self
        .0
        .get_mut(instruction.from - 1)
        .expect(&format!("Unable to get stack {}", instruction.from - 1))
        .0
        .pop()
        .expect("Unable to get top crate from the stack");
      self
        .0
        .get_mut(instruction.to - 1)
        .expect(&format!("Unable to get stack {}", instruction.to - 1))
        .0
        .push(item)
    }
  }

  pub fn apply_instruction_multi_crate(&mut self, instruction: &Instruction) {
    let from_stack = self
      .0
      .get_mut(instruction.from - 1)
      .expect(&format!("Unable to get stack {}", instruction.from - 1));

    let len = from_stack.0.len();
    let mut crates = from_stack.0.drain((len - instruction.times)..).collect::<Vec<Crate>>();

    self
      .0
      .get_mut(instruction.to - 1)
      .expect(&format!("Unable to get stack {}", instruction.to - 1))
      .0
      .append(&mut crates);
  }

  pub fn get_top_crate_names(&self) -> String {
    self
      .0
      .iter()
      .filter(|stack| stack.0.len() > 0)
      .map(|stack| stack.0.last().unwrap().name.as_ref())
      .collect::<Vec<&str>>()
      .join("")
  }

  fn add_crate(&mut self, index: usize, name: &str) {
    match self.0.get_mut(index) {
      Some(stack) => stack.0.push(Crate {
        name: String::from(name),
      }),
      None => {}
    };
  }
}

#[derive(Clone, Debug)]
struct Stack(pub Vec<Crate>);

#[derive(Clone, Debug)]
struct Crate {
  name: String,
}
