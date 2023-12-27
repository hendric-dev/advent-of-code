#[derive(Debug)]
pub struct Movements(pub Vec<Movement>);

impl Movements {
  pub fn new(input: &str) -> Self {
    Self(
      input
        .split("\n")
        .map(|row| {
          let mut split = row.split(" ");
          let direction = split.next().unwrap();
          let times: usize = str::parse(split.next().unwrap()).unwrap();
          (1..=times).map(move |_| match direction {
            "U" => Movement::Up,
            "D" => Movement::Down,
            "L" => Movement::Left,
            "R" => Movement::Right,
            _ => panic!("Failed to parse input, not a valid movement"),
          })
        })
        .flatten()
        .collect(),
    )
  }
}

#[derive(Debug)]
pub enum Movement {
  Up,
  Down,
  Left,
  Right,
}
