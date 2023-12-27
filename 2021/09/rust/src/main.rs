use derive_more::From;
use std::{collections::BTreeMap, fs};

fn main() {
  let input = fs::read_to_string("input.txt").expect("Failed to read input file");
  let heightmap = HeightMap::new(&input);
  let mut basins = heightmap.basins();
  basins.sort_by(|a, b| b.len().cmp(&a.len()));

  println!(
    "Risk level {} ahead!",
    basins.iter().fold(0, |acc, basin| acc
      + basin.iter().min_by(|a, b| a.height.cmp(&b.height)).unwrap().height
      + 1)
  );
  println!(
    "Found three large basins with a size of {}",
    basins.iter().take(3).fold(1, |acc, basin| acc * basin.len())
  );
}

#[derive(Debug, From)]
struct HeightMap(pub BTreeMap<String, Point>);

impl HeightMap {
  pub fn new(input: &str) -> HeightMap {
    let mut map: BTreeMap<String, Point> = BTreeMap::new();
    input.split("\n").enumerate().for_each(|(row, line)| {
      line
        .split("")
        .filter(|height| height.len() > 0)
        .enumerate()
        .for_each(|(column, height)| {
          map.insert(
            format!("{}.{}", row, column),
            Point {
              column,
              height: height.parse::<usize>().unwrap(),
              row,
            },
          );
        })
    });
    HeightMap::from(map)
  }

  pub fn basins(&self) -> Vec<Vec<&Point>> {
    let mut basins: Vec<Vec<&Point>> = vec![];
    self
      .0
      .values()
      .filter(|point| self.is_low_point(point))
      .for_each(|point| {
        let mut basin: Vec<&Point> = vec![];
        let mut higher_points: Vec<&Point> = vec![point];

        while higher_points.len() > 0 {
          let p = higher_points.pop().unwrap();
          if !basin.contains(&p) {
            basin.push(p);
          }
          self
            .get_higher_adjacent_points(p)
            .iter()
            .filter(|point| point.height < 9)
            .for_each(|point| higher_points.push(*point));
        }

        basins.push(basin);
      });

    basins
  }

  fn get_adjacent_points(&self, point: &Point) -> Vec<&Point> {
    let row = point.row as i32;
    let column = point.column as i32;
    let top = self.0.get(&format!("{}.{}", row - 1, column));
    let bottom = self.0.get(&format!("{}.{}", row + 1, column));
    let left = self.0.get(&format!("{}.{}", row, column - 1));
    let right = self.0.get(&format!("{}.{}", row, column + 1));

    [top, bottom, left, right]
      .iter()
      .filter(|point| point.is_some())
      .map(|point| point.unwrap())
      .collect()
  }

  fn get_higher_adjacent_points(&self, point: &Point) -> Vec<&Point> {
    self
      .get_adjacent_points(point)
      .iter()
      .filter(|value| value.height > point.height)
      .map(|value| *value)
      .collect()
  }

  fn is_low_point(&self, point: &Point) -> bool {
    self
      .get_adjacent_points(point)
      .iter()
      .all(|adjacent| adjacent.height > point.height)
  }
}

#[derive(Debug, PartialEq)]
struct Point {
  column: usize,
  height: usize,
  row: usize,
}
