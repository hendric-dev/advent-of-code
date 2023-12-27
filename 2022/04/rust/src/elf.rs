#[derive(Debug)]
pub struct Elf {
  pub sections: Vec<i32>,
}

impl Elf {
  pub fn from_input(input: &str) -> Self {
    let mut split = input.split("-");
    let (start, end) = (
      split.next().unwrap().parse::<i32>().unwrap(),
      split.next().unwrap().parse::<i32>().unwrap(),
    );
    let sections: Vec<i32> = (start..=end).map(|section| section).collect();
    Elf { sections }
  }
}
