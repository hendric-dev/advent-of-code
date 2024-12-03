use regex::Regex;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");

  let mut sum = Regex::new(r"(mul\(\d+,\d+\))")
    .expect("Failed to build operations regex")
    .find_iter(&input)
    .map(|value| value.as_str())
    .map(capture_operands)
    .map(parse_to_int)
    .map(multiply)
    .sum::<i32>();

  println!("[Part 1] 🤖 Beep, Boop... memory restored! summed up all multiplications to {sum}.");

  let mut enabled = true;

  sum = Regex::new(r"(don't\(\)|do\(\)|mul\(\d+,\d+\))")
    .expect("Failed to build operations regex")
    .find_iter(&input)
    .map(|value| value.as_str())
    .map(|value| match value {
      "do()" => {
        enabled = true;
        None
      }
      "don't()" => {
        enabled = false;
        None
      }
      _ => {
        if enabled {
          Some(value)
        } else {
          None
        }
      }
    })
    .filter(defined)
    .map(unwrap)
    .map(capture_operands)
    .map(parse_to_int)
    .map(multiply)
    .sum::<i32>();

  println!("[Part 2] 🤖 Beep, Boop... memory restored even better! summed up all multiplications to {sum}.");
}

fn capture_operands(operation: &str) -> (String, String) {
  let regex = Regex::new(r"\d+").expect("Failed to build operands regex");
  let mut matches = regex.find_iter(operation).map(|value| value.as_str());
  (
    matches.next().expect("Failed to capture first operand").to_owned(),
    matches.next().expect("Failed to capture second operand").to_owned(),
  )
}

fn defined<T>(value: &Option<T>) -> bool {
  value.is_some()
}

fn multiply((a, b): (i32, i32)) -> i32 {
  a * b
}

fn parse_to_int((a, b): (String, String)) -> (i32, i32) {
  (
    str::parse::<i32>(&a).expect("Failed to convert first operand to integer"),
    str::parse::<i32>(&b).expect("Failed to convert second operand to integer"),
  )
}

fn unwrap<T>(value: Option<T>) -> T {
  value.unwrap()
}
