#[derive(Clone, Debug, PartialEq)]
pub struct Item {
  pub item_type: String,
  pub priority: i32,
}

impl Item {
  pub fn new(item_type: &str) -> Self {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let priority = (alphabet.find(item_type).unwrap() + 1) as i32;

    Self {
      item_type: String::from(item_type),
      priority,
    }
  }
}
