use derive_more::From;
use std::{collections::BTreeMap, fs};

fn main() {
  let input = fs::read_to_string("input.txt").expect("Failed to read input file");
  let mut lanternfish: SchoolOfLanternfish = SchoolOfLanternfish::from(BTreeMap::new());
  input
    .split(',')
    .map(|value| value.parse::<i64>().unwrap())
    .for_each(|value| match lanternfish.0.get_mut(&value) {
      Some(fish) => {
        *fish += 1;
      }
      None => {
        lanternfish.0.insert(value, 1);
      }
    });

  (1..=80).for_each(|_| lanternfish.advance_one_day());
  println!("After 80 days the school contains {} lanternfish", lanternfish.count());

  (81..=256).for_each(|_| lanternfish.advance_one_day());
  println!("After 256 days the school contains {} lanternfish", lanternfish.count());
}

#[derive(From)]
struct SchoolOfLanternfish(BTreeMap<i64, i64>);

impl SchoolOfLanternfish {
  pub fn advance_one_day(&mut self) {
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();
    self.0.iter().for_each(|(key, value)| {
      if *key == 0 {
        map.insert(6, *value);
        map.insert(8, *value);
      } else {
        match map.get_mut(&(key - 1)) {
          Some(fish) => {
            *fish += *value;
          }
          None => {
            map.insert(key - 1, *value);
          }
        }
      }
    });
    self.0 = map;
  }

  pub fn count(&self) -> i64 {
    self.0.values().sum::<i64>()
  }
}
