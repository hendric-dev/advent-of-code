use std::slice::Iter;

#[derive(Debug)]
pub struct Race {
  pub time: i64,
  pub record_distance: i64,
}

#[derive(Debug)]
pub struct Races(Vec<Race>);

impl Races {
  pub fn from_input(input: &str) -> Self {
    let times = parse_input(input, "Time:");
    let distances = parse_input(input, "Distance:");

    Self(
      times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, record_distance)| Race { time, record_distance })
        .collect(),
    )
  }

  pub fn from_input_as_single_race(input: &str) -> Self {
    let time = parse_input_as_single_race(input, "Time:");
    let record_distance = parse_input_as_single_race(input, "Distance:");

    Self(vec![Race { time, record_distance }])
  }

  pub fn iter(&self) -> Iter<'_, Race> {
    self.0.iter()
  }

  pub fn multiply_record_breaking_button_presses(&self) -> usize {
    self
      .iter()
      .map(|race| race.record_breaking_button_presses().len())
      .fold(1, |acc, x| acc * x)
  }
}

impl Race {
  pub fn record_breaking_button_presses(&self) -> Vec<i64> {
    let mut record_breakers = vec![];
    for time in 0..=self.time {
      let distance = time * (self.time - time);
      if distance > self.record_distance {
        record_breakers.push(time);
      }
    }

    record_breakers
  }
}

fn parse_input(input: &str, predicate: &str) -> Vec<i64> {
  input
    .split("\n")
    .find(|line| line.starts_with(predicate))
    .expect(&format!("Failed to find input line for predicate {predicate}"))
    .split_whitespace()
    .skip(1)
    .map(|time| time.parse::<i64>().expect("Failed to parse input string into integer"))
    .collect()
}

fn parse_input_as_single_race(input: &str, predicate: &str) -> i64 {
  input
    .split("\n")
    .find(|line| line.starts_with(predicate))
    .expect(&format!("Failed to find input line for predicate {predicate}"))
    .split_whitespace()
    .skip(1)
    .collect::<Vec<&str>>()
    .join("")
    .parse::<i64>()
    .expect("Failed to parse input string into integer")
}
