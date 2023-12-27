use std::{collections::VecDeque, fs::read_to_string};

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let mut cpu = CPU::new(&input);
  let mut signal_strength = 0;
  println!("Checking the signal strength at the cycles 20, 60, 100, 140, 180 and 220...");
  [20, 60, 100, 140, 180, 220].iter().for_each(|cycle| {
    cpu.advance_to_cycle(*cycle);
    signal_strength += cpu.x * cycle;
  });
  println!("Got it! -> {signal_strength}");
  println!();
  cpu.advance_to_cycle(240);
  cpu.draw_crt();
}

struct CPU {
  crt: String,
  crt_position: i32,
  cycle: i32,
  instructions: VecDeque<String>,
  x: i32,
}

impl CPU {
  pub fn new(input: &str) -> Self {
    Self {
      crt: String::from(""),
      crt_position: 0,
      cycle: 1,
      instructions: input.split("\n").map(|instruction| String::from(instruction)).collect(),
      x: 1,
    }
  }

  pub fn advance_to_cycle(&mut self, cycle: i32) {
    while self.cycle < cycle {
      if (cycle - self.cycle) <= self.cycles_for_next_instruction() {
        break;
      }
      self.execute_next_instruction();
    }
  }

  pub fn draw_crt(&self) {
    self
      .crt
      .chars()
      .collect::<Vec<char>>()
      .chunks(40)
      .map(|c| c.iter().collect::<String>())
      .collect::<Vec<String>>()
      .iter()
      .for_each(|chunk| {
        println!("{}", chunk);
      });
  }

  fn draw_pixel(&mut self) {
    if [self.x - 1, self.x, self.x + 1].contains(&self.crt_position) {
      self.crt += "#";
    } else {
      self.crt += ".";
    }
    self.crt_position = (self.crt_position + 1) % 40;
  }

  fn execute_next_instruction(&mut self) {
    match self.instructions.pop_front() {
      Some(instruction) => {
        let mut split = instruction.split(" ");
        match split.next().unwrap() {
          "addx" => {
            self.draw_pixel();
            self.draw_pixel();
            self.cycle += 2;
            self.x += str::parse::<i32>(split.next().unwrap()).unwrap();
          }
          _ => {
            self.draw_pixel();
            self.cycle += 1;
          }
        }
      }
      None => panic!("Failed to get next instruction"),
    };
  }

  fn cycles_for_next_instruction(&self) -> i32 {
    match self.instructions.front() {
      Some(instruction) => match instruction.as_ref() {
        "addx" => 2,
        _ => 1,
      },
      None => 0,
    }
  }
}
