mod signal;

use signal::Signal;
use std::fs;

fn main() {
  let content = fs::read_to_string("input.txt").expect("Failed to read input file");
  let mut signals: Vec<Signal> = content.split("\n").map(|line| Signal::new(line)).collect();
  let decoded: Vec<String> = signals.iter_mut().map(|signal| signal.decode()).collect();

  println!(
    "The numbers 1, 4, 7 and 8 occur exactly {} times.",
    decoded.iter().fold(0, |acc, output| acc
      + output
        .split("")
        .filter(|value| value.len() > 0 && "1478".contains(value))
        .count())
  );

  println!(
    "Sum of decoded output: {}",
    decoded.iter().map(|value| value.parse::<i32>().unwrap()).sum::<i32>()
  );
}
