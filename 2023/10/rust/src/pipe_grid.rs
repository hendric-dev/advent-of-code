use std::collections::HashMap;

#[derive(Debug)]
pub struct PipeGrid(HashMap<(i32, i32), Pipe>);

#[derive(Debug)]
pub struct Pipe {
  shape: String,
  distance: Option<i32>,
  next: Option<(i32, i32)>,
}

impl PipeGrid {
  pub fn from(input: &str) -> Self {
    input
      .lines()
      .enumerate()
      .fold(Self(HashMap::new()), |mut grid, (y, line)| {
        line.chars().enumerate().for_each(|(x, pipe)| {
          grid.insert(x as i32, y as i32, &pipe.to_string());
        });
        grid
      })
  }

  pub fn measure_loop(&mut self) -> i32 {
    let (starting_point_x, starting_point_y) = self.starting_point().expect("No starting point found");
    let mut forward_x = starting_point_x;
    let mut forward_y = starting_point_y;
    let mut backward_x = starting_point_x;
    let mut backward_y = starting_point_y;
    let mut distance = 0;

    self.0.get_mut(&(starting_point_x, starting_point_y)).unwrap().distance = Some(distance);

    loop {
      distance += 1;

      match self.visit_connecting_pipe(forward_x, forward_y, distance) {
        Some((x, y)) => {
          self.0.get_mut(&(forward_x, forward_y)).unwrap().next = Some((x, y));
          forward_x = x;
          forward_y = y;
        }
        None => {
          self.0.get_mut(&(forward_x, forward_y)).unwrap().next = Some((backward_x, backward_y));
          break;
        }
      }

      match self.visit_connecting_pipe(backward_x, backward_y, distance) {
        Some((x, y)) => {
          self.0.get_mut(&(x, y)).unwrap().next = Some((backward_x, backward_y));
          backward_x = x;
          backward_y = y;
        }
        None => {
          self.0.get_mut(&(forward_x, forward_y)).unwrap().next = Some((backward_x, backward_y));
          break;
        }
      }
    }

    distance
  }

  pub fn count_enclosed_tiles(&self) -> i32 {
    let mut corners = self.get_corners();
    corners.push(corners[0]);

    /*
    I solved part 2 by scanning each horizontal line from left to right as follows: For each line:

    Initialize boolean variable 'inside loop' to false

    Change state in one of the following cases:

        a vertical bar of the loop

        a NW loop corner if the previous was a SE loop corner

        a SW loop corner if the previous was a NE loop corner

    Mark a point as inside loop if the variable is true.
    */

    let area = corners
      .windows(2)
      .map(|pair| {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];
        (x1 * y2) - (x2 * y1)
      })
      .fold(0.0, |acc, x| acc + f64::from(x))
      .abs()
      / 2.0;

    area.round() as i32
  }

  fn get_corners(&self) -> Vec<(i32, i32)> {
    let mut corners: Vec<(i32, i32)> = vec![];
    let starting_point = self.starting_point().expect("No starting point found");
    corners.push(starting_point);
    let mut current = self.0.get(&starting_point).unwrap().next.unwrap();

    while current != starting_point {
      let pipe = self
        .0
        .get(&current)
        .expect(&format!("Failed to get pipe at {},{}", current.0, current.1));

      if pipe.shape == "S" || pipe.shape == "7" || pipe.shape == "F" || pipe.shape == "J" || pipe.shape == "L" {
        corners.push(current);
      }
      current = pipe.next.expect("Failed to get next part of the main pipe loop");
    }

    corners
  }

  fn has_unvisited_connecting_pipe_north(&self, x: i32, y: i32) -> bool {
    let current_pipe = self.0.get(&(x, y)).expect(&format!("No pipe found at {x}, {y}"));
    match self.0.get(&(x, y - 1)) {
      Some(pipe) => {
        pipe.distance.is_none()
          && (pipe.shape == "S" || pipe.shape == "|" || pipe.shape == "7" || pipe.shape == "F")
          && (current_pipe.shape == "S"
            || current_pipe.shape == "|"
            || current_pipe.shape == "L"
            || current_pipe.shape == "J")
      }
      None => false,
    }
  }

  fn has_unvisited_connecting_pipe_east(&self, x: i32, y: i32) -> bool {
    let current_pipe = self.0.get(&(x, y)).expect(&format!("No pipe found at {x}, {y}"));
    match self.0.get(&(x + 1, y)) {
      Some(pipe) => {
        pipe.distance.is_none()
          && (pipe.shape == "S" || pipe.shape == "-" || pipe.shape == "J" || pipe.shape == "7")
          && (current_pipe.shape == "S"
            || current_pipe.shape == "-"
            || current_pipe.shape == "F"
            || current_pipe.shape == "L")
      }
      None => false,
    }
  }

  fn has_unvisited_connecting_pipe_south(&self, x: i32, y: i32) -> bool {
    let current_pipe = self.0.get(&(x, y)).expect(&format!("No pipe found at {x}, {y}"));
    match self.0.get(&(x, y + 1)) {
      Some(pipe) => {
        pipe.distance.is_none()
          && (pipe.shape == "S" || pipe.shape == "|" || pipe.shape == "J" || pipe.shape == "L")
          && (current_pipe.shape == "S"
            || current_pipe.shape == "|"
            || current_pipe.shape == "F"
            || current_pipe.shape == "7")
      }
      None => false,
    }
  }

  fn has_unvisited_connecting_pipe_west(&self, x: i32, y: i32) -> bool {
    let current_pipe = self.0.get(&(x, y)).expect(&format!("No pipe found at {x}, {y}"));
    match self.0.get(&(x - 1, y)) {
      Some(pipe) => {
        pipe.distance.is_none()
          && (pipe.shape == "S" || pipe.shape == "-" || pipe.shape == "F" || pipe.shape == "L")
          && (current_pipe.shape == "S"
            || current_pipe.shape == "-"
            || current_pipe.shape == "J"
            || current_pipe.shape == "7")
      }
      None => false,
    }
  }

  fn insert(&mut self, x: i32, y: i32, pipe: &str) {
    self.0.insert(
      (x, y),
      Pipe {
        shape: pipe.to_string(),
        distance: None,
        next: None,
      },
    );
  }

  fn starting_point(&self) -> Option<(i32, i32)> {
    let start = self.0.iter().find(|(_, pipe)| pipe.shape == "S");
    match start {
      Some(((x, y), _)) => Some((*x, *y)),
      None => None,
    }
  }

  fn visit_connecting_pipe(&mut self, x: i32, y: i32, distance: i32) -> Option<(i32, i32)> {
    let next: Option<(i32, i32)>;

    if self.has_unvisited_connecting_pipe_north(x, y) {
      next = Some((x, y - 1));
    } else if self.has_unvisited_connecting_pipe_east(x, y) {
      next = Some((x + 1, y));
    } else if self.has_unvisited_connecting_pipe_south(x, y) {
      next = Some((x, y + 1));
    } else if self.has_unvisited_connecting_pipe_west(x, y) {
      next = Some((x - 1, y));
    } else {
      return None;
    }

    self.0.get_mut(&(next.unwrap().0, next.unwrap().1)).unwrap().distance = Some(distance);
    next
  }
}
