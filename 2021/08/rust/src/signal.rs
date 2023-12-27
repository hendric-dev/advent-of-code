use std::collections::BTreeMap;

pub struct Signal {
  decryption: BTreeMap<String, String>,
  pattern: Vec<String>,
  output: Vec<String>,
}

impl Signal {
  pub fn new(line: &str) -> Signal {
    let split = line.split(" | ").collect::<Vec<&str>>();
    let formatted = split
      .iter()
      .map(|value| value.split(" ").map(|value| String::from(value)).collect())
      .collect::<Vec<Vec<String>>>();

    Signal {
      decryption: BTreeMap::new(),
      pattern: formatted.get(0).unwrap().to_owned(),
      output: formatted.get(1).unwrap().to_owned(),
    }
  }

  pub fn decode(&mut self) -> String {
    self.decode_unique_pattern();
    self.pattern.iter().for_each(|pattern| {
      match pattern.len() {
        5 => {
          if includes(&self.decryption, &pattern, "7") {
            self.decryption.insert(String::from("3"), String::from(pattern));
          } else if !includes(&self.decryption, &pattern, "7")
            && self
              .decryption
              .get("4")
              .unwrap()
              .split("")
              .filter(|char| char.len() > 0 && pattern.contains(char))
              .count()
              == 2
          {
            self.decryption.insert(String::from("2"), String::from(pattern));
          } else {
            self.decryption.insert(String::from("5"), String::from(pattern));
          }
        }
        6 => {
          if includes(&self.decryption, &pattern, "4") {
            self.decryption.insert(String::from("9"), String::from(pattern));
          } else if !includes(&self.decryption, &pattern, "4") && !includes(&self.decryption, &pattern, "1") {
            self.decryption.insert(String::from("6"), String::from(pattern));
          } else {
            self.decryption.insert(String::from("0"), String::from(pattern));
          }
        }
        _ => {}
      };
    });

    self
      .output
      .iter()
      .map(|pattern| {
        self
          .decryption
          .iter()
          .find(|(_key, value)| {
            pattern
              .split("")
              .all(|char| value.len() == pattern.len() && value.contains(char))
          })
          .unwrap()
          .0
          .as_ref()
      })
      .collect::<Vec<&str>>()
      .join("")
  }

  fn decode_unique_pattern(&mut self) {
    self.pattern.iter().for_each(|pattern| {
      match pattern.len() {
        2 => {
          self.decryption.insert(String::from("1"), String::from(pattern));
        }
        3 => {
          self.decryption.insert(String::from("7"), String::from(pattern));
        }
        4 => {
          self.decryption.insert(String::from("4"), String::from(pattern));
        }
        7 => {
          self.decryption.insert(String::from("8"), String::from(pattern));
        }
        _ => {}
      };
    });
  }
}

fn includes(decryption: &BTreeMap<String, String>, pattern: &str, key: &str) -> bool {
  decryption
    .get(key)
    .unwrap()
    .split("")
    .all(|char| pattern.contains(char))
}
