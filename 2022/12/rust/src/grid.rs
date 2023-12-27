use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub struct Grid(BTreeMap<(Row, Col), Node>);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Row(usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Col(usize);

#[derive(Clone, Debug)]
pub struct Node {
  row: Row,
  col: Col,
  height: usize,
  distance: usize,
  is_start: bool,
  is_goal: bool,
}

impl Grid {
  pub fn new(input: &str) -> Self {
    let alphabet = String::from_utf8((b'a'..=b'z').collect()).expect("Failed to generate alphabet");
    let mut grid = Self(BTreeMap::new());
    input.split("\n").enumerate().for_each(|(row, line)| {
      line.chars().enumerate().for_each(|(col, height)| {
        grid.0.insert(
          (Row(row), Col(col)),
          Node {
            row: Row(row),
            col: Col(col),
            height: match alphabet.find(height) {
              Some(value) => value,
              None => {
                if height == 'S' {
                  0
                } else {
                  25
                }
              }
            },
            distance: 0,
            is_start: height == 'S',
            is_goal: height == 'E',
          },
        );
      });
    });
    grid
  }

  pub fn discover(&mut self) {
    let mut discoverable_nodes = VecDeque::from([self.get_goal_node().clone()]);
    let mut visited_nodes: BTreeMap<(Row, Col), bool> = BTreeMap::new();

    while !discoverable_nodes.is_empty() {
      let current = discoverable_nodes.pop_front().unwrap();
      let mut neighbors = self.neighbors(&current);
      neighbors.iter_mut().for_each(|neighbor| {
        if !neighbor.is_goal && neighbor.is_climbable(&current) && neighbor.found_a_shorter_way(&current) {
          neighbor.distance = current.distance + 1;
          if !visited_nodes.contains_key(&(neighbor.row, neighbor.col)) {
            discoverable_nodes.push_back(neighbor.clone());
          }
        }
      });
      visited_nodes.insert((current.row, current.col), true);
    }
  }

  pub fn get_distance(&self) -> usize {
    self
      .0
      .values()
      .find(|node| node.is_start)
      .expect("Failed to find starting node")
      .distance
  }

  pub fn get_distance_multi_start(&self) -> usize {
    self
      .0
      .values()
      .filter(|node| node.height == 0 && node.distance > 0)
      .map(|node| node.distance)
      .min()
      .unwrap()
  }

  pub fn get_goal_node(&self) -> &Node {
    self
      .0
      .values()
      .find(|node| node.is_goal)
      .expect("Failed to find goal node")
  }

  fn neighbors(&mut self, current: &Node) -> Vec<&mut Node> {
    self
      .0
      .range_mut(
        (
          Row(if current.row.0 == 0 { 0 } else { current.row.0 - 1 }),
          Col(current.col.0),
        )..=(Row(current.row.0 + 1), Col(current.col.0)),
      )
      .filter(|(_, node)| {
        let is_up = current.row.0 > 0 && node.col == current.col && node.row.0 == current.row.0 - 1;
        let is_down = node.col == current.col && node.row.0 == current.row.0 + 1;
        let is_left = current.col.0 > 0 && node.row == current.row && node.col.0 == current.col.0 - 1;
        let is_right = node.row == current.row && node.col.0 == current.col.0 + 1;

        is_up || is_down || is_left || is_right
      })
      .map(|(_, node)| node)
      .collect()
  }
}

impl Node {
  fn found_a_shorter_way(&self, current: &Node) -> bool {
    self.distance == 0 || self.distance > current.distance + 1
  }

  fn is_climbable(&self, current: &Node) -> bool {
    self.height + 1 >= current.height
  }
}
