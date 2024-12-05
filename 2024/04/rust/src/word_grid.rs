use std::collections::BTreeMap;

#[derive(Debug)]
pub struct WordGrid(BTreeMap<(i32, i32), String>);

enum Direction {
  Down,
  DownLeft,
  DownRight,
  Left,
  Right,
  Up,
  UpLeft,
  UpRight,
}

impl WordGrid {
  pub fn from_input(input: &str) -> WordGrid {
    let mut grid: BTreeMap<(i32, i32), String> = BTreeMap::new();
    input.split("\n").enumerate().for_each(|(row, text)| {
      text.chars().enumerate().for_each(|(col, char)| {
        grid.insert((row as i32, col as i32), char.to_string());
      });
    });

    Self(grid)
  }

  pub fn count_xmas(&self) -> usize {
    self
      .0
      .iter()
      .filter(|((_, _), char)| char.as_str() == "X")
      .map(|((row, col), _)| {
        vec![
          self.is_xmas(*row, *col, Direction::Down),
          self.is_xmas(*row, *col, Direction::DownLeft),
          self.is_xmas(*row, *col, Direction::DownRight),
          self.is_xmas(*row, *col, Direction::Left),
          self.is_xmas(*row, *col, Direction::Right),
          self.is_xmas(*row, *col, Direction::Up),
          self.is_xmas(*row, *col, Direction::UpLeft),
          self.is_xmas(*row, *col, Direction::UpRight),
        ]
        .iter()
        .filter(|result| *result == &true)
        .count()
      })
      .sum()
  }

  pub fn count_x_mas(&self) -> usize {
    self
      .0
      .iter()
      .filter(|((_, _), char)| char.as_str() == "A")
      .filter(|((row, col), _)| self.is_x_mas(*row, *col))
      .count()
  }

  fn get(&self, row: i32, col: i32) -> Option<String> {
    self.0.get(&(row, col)).cloned()
  }

  fn is_xmas(&self, row: i32, col: i32, direction: Direction) -> bool {
    let a: Option<String>;
    let b: Option<String>;
    let c: Option<String>;

    match direction {
      Direction::Down => {
        a = self.get(row + 1, col);
        b = self.get(row + 2, col);
        c = self.get(row + 3, col);
      }
      Direction::DownLeft => {
        a = self.get(row + 1, col - 1);
        b = self.get(row + 2, col - 2);
        c = self.get(row + 3, col - 3);
      }
      Direction::DownRight => {
        a = self.get(row + 1, col + 1);
        b = self.get(row + 2, col + 2);
        c = self.get(row + 3, col + 3);
      }
      Direction::Left => {
        a = self.get(row, col - 1);
        b = self.get(row, col - 2);
        c = self.get(row, col - 3);
      }
      Direction::Right => {
        a = self.get(row, col + 1);
        b = self.get(row, col + 2);
        c = self.get(row, col + 3);
      }
      Direction::Up => {
        a = self.get(row - 1, col);
        b = self.get(row - 2, col);
        c = self.get(row - 3, col);
      }
      Direction::UpLeft => {
        a = self.get(row - 1, col - 1);
        b = self.get(row - 2, col - 2);
        c = self.get(row - 3, col - 3);
      }
      Direction::UpRight => {
        a = self.get(row - 1, col + 1);
        b = self.get(row - 2, col + 2);
        c = self.get(row - 3, col + 3);
      }
    }

    format!(
      "X{}{}{}",
      a.unwrap_or_default(),
      b.unwrap_or_default(),
      c.unwrap_or_default()
    ) == "XMAS"
  }

  fn is_x_mas(&self, row: i32, col: i32) -> bool {
    let diagonal_pair_a = format!(
      "{}{}",
      self.get(row - 1, col - 1).unwrap_or_default(),
      self.get(row + 1, col + 1).unwrap_or_default()
    );
    let diagonal_pair_b = format!(
      "{}{}",
      self.get(row + 1, col - 1).unwrap_or_default(),
      self.get(row - 1, col + 1).unwrap_or_default()
    );

    (diagonal_pair_a == "MS" || diagonal_pair_a == "SM") && (diagonal_pair_b == "MS" || diagonal_pair_b == "SM")
  }
}
