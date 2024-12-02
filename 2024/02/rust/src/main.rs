use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let reports: Vec<Report> = input.split("\n").map(|report| Report::from_input(report)).collect();
  let save_reports_without_problem_dampener = reports.iter().filter(|report| report.is_safe(false)).count();
  let save_reports_with_problem_dampener = reports.iter().filter(|report| report.is_safe(true)).count();

  println!("[Part 1] 🤖 Beep, Boop... calculated {save_reports_without_problem_dampener} save reports when using no problem dampener");
  println!("[Part 2] 🤖 Beep, Boop... calculated {save_reports_with_problem_dampener} save reports when using a problem dampener");
}

#[derive(Debug)]
struct Report(Vec<i32>);

impl Report {
  pub fn from_input(input: &str) -> Report {
    Self(
      input
        .split(" ")
        .map(|value| str::parse::<i32>(value).expect("Failed to convert input value to integer"))
        .collect(),
    )
  }

  pub fn is_safe(&self, with_problem_dampener: bool) -> bool {
    if with_problem_dampener {
      (0..self.0.len()).any(|index| {
        let report: Vec<i32> = self
          .0
          .iter()
          .enumerate()
          .filter(|(i, _)| i != &index)
          .map(|(_, v)| *v)
          .collect();

        return is_safe(&report);
      })
    } else {
      is_safe(&self.0)
    }
  }
}

fn is_safe(report: &Vec<i32>) -> bool {
  let pairs = || report.iter().zip(report.iter().skip(1));
  let increasing = |(a, b): (&i32, &i32)| -> bool { a < b };
  let decreasing = |(a, b): (&i32, &i32)| -> bool { a > b };
  let max_distance = |max: i32| move |(a, b): (&i32, &i32)| -> bool { (a - b).abs() <= max };

  pairs().all(max_distance(3)) && (pairs().all(increasing) || pairs().all(decreasing))
}
