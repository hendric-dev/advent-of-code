use crate::commands::Command;
use std::collections::BTreeMap;

pub struct Directories {
  pwd: Vec<String>,
  tree: BTreeMap<String, Directory>,
}

impl Directories {
  pub fn new() -> Self {
    Self {
      pwd: vec![],
      tree: BTreeMap::from([(String::from("/"), Directory::default())]),
    }
  }

  pub fn execute_command(&mut self, command: &Command) {
    if command.instruction.starts_with("cd") {
      self.run_cd_command(command);
    }
    if command.instruction == "ls" {
      self.run_ls_command(command);
    }
  }

  pub fn size_of_smallest_directory_above(&self, size: &i32) -> i32 {
    *self.total_sizes().iter().filter(|value| *value >= size).min().unwrap()
  }

  pub fn sum_of_directories_smaller_than(&self, size: &i32) -> i32 {
    self.total_sizes().iter().filter(|value| *value <= size).sum::<i32>()
  }

  pub fn used_space(&self) -> i32 {
    self.tree.values().map(|directory| directory.size()).sum()
  }

  fn run_cd_command(&mut self, command: &Command) {
    let name = command.instruction.replace("cd ", "");
    match name.as_ref() {
      "/" => {
        self.pwd = vec![name];
      }
      ".." => {
        self.pwd.pop();
      }
      _ => {
        self.pwd.push(name);
      }
    }
  }

  fn run_ls_command(&mut self, command: &Command) {
    command.output.iter().for_each(|output| {
      if output.starts_with("dir") {
        let name = output.replace("dir ", "");
        self.tree.insert(self.pwd.join("") + &name, Directory::default());
      } else {
        self
          .tree
          .get_mut(&self.pwd.join(""))
          .expect("Failed to get directory")
          .files
          .push(File {
            size: str::parse::<i32>(output.split(" ").next().unwrap()).unwrap(),
          })
      }
    });
  }

  fn total_sizes(&self) -> Vec<i32> {
    self
      .tree
      .keys()
      .map(|path| {
        self
          .tree
          .keys()
          .filter(|key| key.starts_with(path))
          .map(|key| self.tree.get(key).unwrap().size())
          .sum::<i32>()
      })
      .collect()
  }
}

#[derive(Clone, Debug, Default)]
pub struct Directory {
  files: Vec<File>,
}

impl Directory {
  pub fn size(&self) -> i32 {
    self.files.iter().map(|file| file.size).sum()
  }
}

#[derive(Clone, Debug)]
pub struct File {
  size: i32,
}
