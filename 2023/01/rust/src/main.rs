use std::fs::read_to_string;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let calibration1: i32 = input.lines().map(|line| extract_first_and_last_digit(line)).sum();
  println!("[Part 1] The calibration value is: {}", calibration1);

  let calibration2: i32 = input
    .lines()
    .map(|line| extract_first_and_last_digit(&replace_spelled_out_digits(line)))
    .sum();
  println!("[Part 2] The calibration value is: {}", calibration2);
}

fn extract_first_and_last_digit(line: &str) -> i32 {
  let digits = line
    .chars()
    .filter(|c| c.is_digit(10))
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join("");

  let first_digit = digits.chars().nth(0).expect("Failed to get first digit");
  let last_digit = digits.chars().nth(digits.len() - 1).expect("Failed to get last digit");
  let digit = format!("{first_digit}{last_digit}")
    .parse::<i32>()
    .expect("Failed to convert to integer");
  digit
}

fn replace_spelled_out_digits(line: &str) -> String {
  let mut replaced_line = line.replace("one", "one1one");
  replaced_line = replaced_line.replace("two", "two2two");
  replaced_line = replaced_line.replace("three", "three3three");
  replaced_line = replaced_line.replace("four", "four4four");
  replaced_line = replaced_line.replace("five", "five5five");
  replaced_line = replaced_line.replace("six", "six6six");
  replaced_line = replaced_line.replace("seven", "seven7seven");
  replaced_line = replaced_line.replace("eight", "eight8eight");
  replaced_line.replace("nine", "nine9nine")
}
