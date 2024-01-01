use std::collections::VecDeque;

#[derive(Debug)]
pub struct Dataset(Vec<History>);

#[derive(Debug)]
pub struct History(Vec<VecDeque<i32>>);

impl Dataset {
  pub fn from_input(input: &str) -> Self {
    Self(
      input
        .lines()
        .map(|line| History::from_line(line))
        .collect::<Vec<History>>(),
    )
  }

  pub fn calculate_sequences(&mut self) {
    self.0.iter_mut().for_each(|history| history.calculate_sequences());
  }

  pub fn extrapolate(&mut self, direction: &str) -> Vec<i32> {
    self
      .0
      .iter_mut()
      .map(|history| match direction {
        "end" => history.extrapolate_end(),
        "front" => history.extrapolate_front(),
        _ => panic!("Invalid direction"),
      })
      .collect()
  }
}

impl History {
  pub fn from_line(line: &str) -> Self {
    Self(vec![line
      .split_whitespace()
      .map(|number| number.parse::<i32>().expect("Failed to parse number"))
      .collect::<VecDeque<i32>>()])
  }

  pub fn calculate_sequences(&mut self) {
    while !self.found_zero_sequence() {
      let last_sequence = self.0.last().expect("Failed to get last sequence");
      self.0.push(
        last_sequence
          .iter()
          .zip(last_sequence.iter().skip(1))
          .map(|(&a, &b)| b - a)
          .collect(),
      );
    }
  }

  pub fn extrapolate_end(&mut self) -> i32 {
    let mut previous_end_value = 0;
    let last_sequence = self.0.last_mut().expect("Failed to get last sequence");
    last_sequence.push_back(previous_end_value);

    self.0.iter_mut().rev().skip(1).for_each(|sequence| {
      let last_value = sequence.back().expect("Failed to get last value").clone();
      let new_value = last_value + previous_end_value;
      sequence.push_back(new_value);
      previous_end_value = new_value;
    });

    previous_end_value
  }

  pub fn extrapolate_front(&mut self) -> i32 {
    let mut previous_front_value = 0;
    let last_sequence = self.0.last_mut().expect("Failed to get last sequence");
    last_sequence.push_front(previous_front_value);

    self.0.iter_mut().rev().skip(1).for_each(|sequence| {
      let first_value = sequence.front().expect("Failed to get first value").clone();
      let new_value = first_value - previous_front_value;
      sequence.push_back(new_value);
      previous_front_value = new_value;
    });

    previous_front_value
  }

  fn found_zero_sequence(&self) -> bool {
    self
      .0
      .last()
      .expect("Failed to get last sequence")
      .iter()
      .all(|value| *value == 0)
  }
}
