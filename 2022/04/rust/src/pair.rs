use crate::elf::Elf;

#[derive(Debug)]
pub struct Pair((Elf, Elf));

impl Pair {
  pub fn from_input(input: &str) -> Self {
    let mut split = input.split(",");
    let (first, second) = (split.next().unwrap(), split.next().unwrap());
    Pair((Elf::from_input(first), Elf::from_input(second)))
  }

  pub fn one_fully_contained(&self) -> bool {
    self
      .first_elf()
      .sections
      .iter()
      .all(|section| self.second_elf().sections.contains(section))
      || self
        .second_elf()
        .sections
        .iter()
        .all(|section| self.first_elf().sections.contains(section))
  }

  pub fn overlaps(&self) -> bool {
    self
      .first_elf()
      .sections
      .iter()
      .any(|section| self.second_elf().sections.contains(section))
  }

  fn first_elf(&self) -> &Elf {
    &self.0 .0
  }
  fn second_elf(&self) -> &Elf {
    &self.0 .1
  }
}
