use crate::{item::Item, rucksack::Rucksack};

#[derive(Debug)]
pub struct Groups(Vec<Group>);

impl Groups {
  pub fn from_input(input: &str, group_size: usize) -> Self {
    Self(
      input
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(group_size)
        .map(|rows| Group::from_input(&rows.join("\n")))
        .collect(),
    )
  }

  pub fn badge_priority_sum(&self) -> i32 {
    self
      .0
      .iter()
      .map(|group| group.find_badge())
      .map(|item| item.priority)
      .sum()
  }

  pub fn duplicates_priority_sum(&self) -> i32 {
    self.0.iter().map(|group| group.duplicates_priority_sum()).sum()
  }
}

#[derive(Debug)]
struct Group(Vec<Rucksack>);

impl Group {
  fn from_input(input: &str) -> Self {
    Self(input.split("\n").map(|row| Rucksack::from_input(row)).collect())
  }

  fn duplicates_priority_sum(&self) -> i32 {
    self
      .0
      .iter()
      .map(|rucksack| rucksack.get_duplicates())
      .map(|duplicates| duplicates.iter().map(|item| item.priority).sum::<i32>())
      .sum()
  }

  fn find_badge(&self) -> Item {
    self
      .0
      .get(0)
      .unwrap()
      .joined_compartments()
      .iter()
      .find(|item| {
        self
          .0
          .iter()
          .skip(1)
          .all(|rucksack| rucksack.first_compartment.contains(item) || rucksack.second_compartment.contains(item))
      })
      .expect("Could not find a badge.")
      .clone()
  }
}
