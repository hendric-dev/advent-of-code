#[derive(Debug)]
pub struct Commands(pub Vec<Command>);

impl Commands {
  pub fn new(input: &str) -> Self {
    Self(input.split("\n").fold(vec![], |mut result, row| {
      if row.starts_with("$") {
        result.push(Command {
          instruction: row.replace("$ ", ""),
          ..Default::default()
        });
      } else {
        result.last_mut().expect("Could not get last command").add_output(row)
      }
      result
    }))
  }
}

#[derive(Debug, Default)]
pub struct Command {
  pub instruction: String,
  pub output: Vec<String>,
}

impl Command {
  fn add_output(&mut self, result: &str) {
    self.output.push(String::from(result));
  }
}
