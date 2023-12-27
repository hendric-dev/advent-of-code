use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Monkeys {
  monkeys: Vec<Monkey>,
  least_common_multiple_divisor: i64,
  with_worry_relief: bool,
}

impl Monkeys {
  pub fn new(input: &str, with_worry_relief: bool) -> Self {
    let mut monkeys = Self {
      monkeys: input
        .split("\n\n")
        .map(|monkey| {
          let split: Vec<&str> = monkey.split("\n").collect();
          let items: VecDeque<Item> = split
            .get(1)
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|c| Item {
              value: str::parse::<i64>(c).unwrap(),
            })
            .collect();
          let operation = String::from(split.get(2).unwrap().split(" = ").last().unwrap());
          let divisble_by: i64 = str::parse(split.get(3).unwrap().split(" divisible by ").last().unwrap()).unwrap();
          let test_successful: i32 =
            str::parse(split.get(4).unwrap().split(" throw to monkey ").last().unwrap()).unwrap();
          let test_failed: i32 = str::parse(split.get(5).unwrap().split(" throw to monkey ").last().unwrap()).unwrap();

          Monkey {
            items,
            items_inspected: 0,
            operation,
            divisble_by,
            test_successful,
            test_failed,
          }
        })
        .collect(),
      least_common_multiple_divisor: 0,
      with_worry_relief,
    };
    monkeys.calculate_least_common_multiple_divisor();
    monkeys
  }

  pub fn monkey_business(&mut self) -> i64 {
    self
      .sorted_by_inspected_items()
      .iter()
      .take(2)
      .map(|m| m.items_inspected)
      .fold(0, |acc, value| {
        if acc == 0 {
          return value;
        }
        acc * value
      })
  }

  pub fn simulate_rounds(&mut self, rounds: i32) {
    for _ in 1..=rounds {
      self.simulate_round();
    }
  }

  pub fn simulate_round(&mut self) {
    let mut map: BTreeMap<i32, VecDeque<Item>> = BTreeMap::new();
    self.monkeys.iter_mut().enumerate().for_each(|(index, monkey)| {
      monkey
        .items
        .append(map.get_mut(&(index as i32)).unwrap_or(&mut VecDeque::from([])));
      monkey.items.clone().iter_mut().for_each(|item| {
        let (operation, operand) = monkey.split_operation();

        if !self.with_worry_relief {
          item.value -= ((item.value / self.least_common_multiple_divisor) - 1) * self.least_common_multiple_divisor;
        }

        match operation {
          "*" => match str::parse::<i64>(operand) {
            Ok(value) => {
              item.value *= value;
            }
            Err(_) => {
              item.value *= item.value;
            }
          },
          "+" => match str::parse::<i64>(operand) {
            Ok(value) => {
              item.value += value;
            }
            Err(_) => item.value += item.value,
          },
          _ => {
            panic!("Unknown operation discovered")
          }
        }

        if self.with_worry_relief {
          item.value /= 3;
        }
        if item.value % monkey.divisble_by == 0 {
          match map.get_mut(&monkey.test_successful) {
            Some(value) => {
              value.push_back(*item);
            }
            None => {
              map.insert(monkey.test_successful, VecDeque::from([*item]));
            }
          };
        } else {
          match map.get_mut(&monkey.test_failed) {
            Some(value) => {
              value.push_back(*item);
            }
            None => {
              map.insert(monkey.test_failed, VecDeque::from([*item]));
            }
          };
        }
        monkey.items_inspected += 1;
      });
      monkey.items.clear();
    });
    for (index, record) in map {
      self.monkeys.get_mut(index as usize).unwrap().items = record;
    }
  }

  pub fn sorted_by_inspected_items(&mut self) -> Vec<Monkey> {
    let mut clone = self.monkeys.clone();
    clone.sort_by(|a, b| b.items_inspected.partial_cmp(&a.items_inspected).unwrap());
    clone
  }

  fn calculate_least_common_multiple_divisor(&mut self) {
    let divisors: Vec<i64> = self.monkeys.iter().map(|m| m.divisble_by).collect();
    let min = *divisors.iter().min().unwrap();
    let end = i64::MAX;

    for value in (min..=end).step_by(min as usize) {
      if divisors.iter().all(|d| value % d == 0) {
        self.least_common_multiple_divisor = value;
        return;
      }
    }
  }
}

#[derive(Clone, Debug)]
pub struct Monkey {
  items: VecDeque<Item>,
  items_inspected: i64,
  operation: String,
  divisble_by: i64,
  test_successful: i32,
  test_failed: i32,
}

impl Monkey {
  fn split_operation(&self) -> (&str, &str) {
    let mut split = self.operation.split(" ").skip(1);
    let operation = split.next().unwrap();
    let operand = split.next().unwrap();

    (operation, operand)
  }
}

#[derive(Clone, Copy, Debug)]
struct Item {
  value: i64,
}
