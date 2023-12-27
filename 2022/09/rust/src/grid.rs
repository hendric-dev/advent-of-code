use crate::movements::Movement;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Grid {
  knots: Vec<Knot>,
  map: BTreeMap<(Row, Col), Point>,
}

#[derive(Clone, Debug)]
pub struct Knot {
  row: Row,
  col: Col,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Row(i32);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Col(i32);

#[derive(Debug)]
pub struct Point {
  visited: bool,
}

impl Grid {
  pub fn new(knots: usize) -> Self {
    Self {
      knots: (1..=knots)
        .map(|_| Knot {
          row: Row(0),
          col: Col(0),
        })
        .collect(),
      map: BTreeMap::from([((Row(0), Col(0)), Point { visited: true })]),
    }
  }

  pub fn draw_visited(&self) {
    let min_row = self.map.keys().map(|(row, _)| row.0).min().unwrap();
    let max_row = self.map.keys().map(|(row, _)| row.0).max().unwrap();
    let min_col = self.map.keys().map(|(_, col)| col.0).min().unwrap();
    let max_col = self.map.keys().map(|(_, col)| col.0).max().unwrap();

    (min_row..=max_row).rev().for_each(|row| {
      println!();
      (min_col..=max_col).for_each(|col| {
        if self.map.contains_key(&(Row(row), Col(col))) {
          print!("#");
        } else {
          print!(".");
        }
      })
    })
  }

  pub fn move_head(&mut self, movement: &Movement) {
    let head = self.knots.first_mut().unwrap();

    match movement {
      Movement::Up => {
        head.row.0 += 1;
      }
      Movement::Down => {
        head.row.0 -= 1;
      }
      Movement::Left => {
        head.col.0 -= 1;
      }
      Movement::Right => {
        head.col.0 += 1;
      }
    }

    let mut predecessor = head.clone();
    self.knots.iter_mut().skip(1).for_each(|knot| {
      knot.adjust(&predecessor);
      predecessor = knot.clone();
    });
    self.map.insert(
      (self.knots.last().unwrap().row, self.knots.last().unwrap().col),
      Point { visited: true },
    );
  }

  pub fn visited_points(&self) -> usize {
    self.map.values().filter(|point| point.visited).count()
  }
}

impl Knot {
  fn adjust(&mut self, predecessor: &Knot) {
    let is_movement_needed = (predecessor.row.0 - self.row.0).abs() >= 2 || (predecessor.col.0 - self.col.0).abs() >= 2;
    if !is_movement_needed {
      return;
    }

    if predecessor.row > self.row {
      self.row.0 += 1;
      if predecessor.col > self.col {
        self.col.0 += 1;
      } else if predecessor.col < self.col {
        self.col.0 -= 1;
      }
    } else if predecessor.row < self.row {
      self.row.0 -= 1;
      if predecessor.col > self.col {
        self.col.0 += 1;
      } else if predecessor.col < self.col {
        self.col.0 -= 1;
      }
    } else if predecessor.col > self.col {
      self.col.0 += 1;
      if predecessor.row > self.row {
        self.row.0 += 1;
      } else if predecessor.row < self.row {
        self.row.0 -= 1;
      }
    } else if predecessor.col < self.col {
      self.col.0 -= 1;
      if predecessor.row > self.row {
        self.row.0 += 1;
      } else if predecessor.row < self.row {
        self.row.0 -= 1;
      }
    }
  }
}
