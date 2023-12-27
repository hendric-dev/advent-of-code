use std::collections::BTreeMap;

pub struct Grid(BTreeMap<(usize, usize), usize>);

impl Grid {
  pub fn new(input: &str) -> Self {
    let mut grid = BTreeMap::new();
    input.split("\n").enumerate().for_each(|(row, item)| {
      item.chars().enumerate().for_each(|(col, tree)| {
        grid.insert((row, col), tree.to_digit(10).unwrap() as usize);
      });
    });

    Self(grid)
  }

  pub fn highest_scenic_score(&self) -> usize {
    let max_row = self.max_row();
    let max_col = max_row;

    self
      .0
      .iter()
      .map(|((row, col), tree)| {
        let mut up = 0;
        for r in (0..=*row).rev().skip(1) {
          up += 1;
          if self.0.get(&(r, *col)).unwrap() >= tree {
            break;
          }
        }
        let mut down = 0;
        for r in (*row..=max_row).skip(1) {
          down += 1;
          if self.0.get(&(r, *col)).unwrap() >= tree {
            break;
          }
        }
        let mut left = 0;
        for c in (0..=*col).rev().skip(1) {
          left += 1;
          if self.0.get(&(*row, c)).unwrap() >= tree {
            break;
          }
        }
        let mut right = 0;
        for c in (*col..=max_col).skip(1) {
          right += 1;
          if self.0.get(&(*row, c)).unwrap() >= tree {
            break;
          }
        }

        up * down * left * right
      })
      .max()
      .unwrap()
  }

  pub fn visible_trees(&self) -> usize {
    let max_row = self.max_row();
    let max_col = max_row;

    self
      .0
      .iter()
      .filter(|((row, col), tree)| {
        let up = (0..*row).all(|r| self.0.get(&(r, *col)).unwrap() < tree);
        let down = (*row..=max_row).all(|r| r == *row || self.0.get(&(r, *col)).unwrap() < tree);
        let left = (0..*col).all(|c| self.0.get(&(*row, c)).unwrap() < tree);
        let right = (*col..=max_col).all(|c| c == *col || self.0.get(&(*row, c)).unwrap() < tree);

        up || down || left || right
      })
      .count()
  }

  fn max_row(&self) -> usize {
    self.0.keys().map(|(row, col)| row + col).max().unwrap() / 2
  }
}
