use crate::item::Item;

#[derive(Debug)]
pub struct Rucksack {
  pub first_compartment: Vec<Item>,
  pub second_compartment: Vec<Item>,
}

impl Rucksack {
  pub fn from_input(input: &str) -> Self {
    let (first_compartment, second_compartment) = input.split_at(input.len() / 2);
    Self {
      first_compartment: first_compartment
        .chars()
        .map(|item_type| Item::new(&item_type.to_string()))
        .collect(),
      second_compartment: second_compartment
        .chars()
        .map(|item_type| Item::new(&item_type.to_string()))
        .collect(),
    }
  }

  pub fn get_duplicates(&self) -> Vec<&Item> {
    let mut items: Vec<&Item> = self
      .first_compartment
      .iter()
      .filter(|item| self.second_compartment.contains(item))
      .collect();
    items.dedup();
    items
  }

  pub fn joined_compartments(&self) -> Vec<Item> {
    let mut joined = self.first_compartment.clone();
    let mut second = self.second_compartment.clone();
    joined.append(&mut second);
    joined
  }
}
