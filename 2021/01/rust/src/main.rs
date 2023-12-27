use std::fs;

fn main() {
  let content = fs::read_to_string("input.txt").expect("Failed to read input file");
  let measurements: Vec<i32> = content.split('\n').map(|value| value.parse().unwrap()).collect();
  part_one(&measurements);
  part_two(&measurements);
}

fn part_one(measurements: &Vec<i32>) {
  let mut increases = 0;
  let mut previous = measurements.get(0).unwrap();

  measurements.iter().for_each(|value| {
    if value > previous {
      increases += 1
    };
    previous = value;
  });

  println!("Part One Status Report: Depth increased {} times!", increases);
}

fn part_two(measurements: &Vec<i32>) {
  let mut increases = 0;
  let mut previous: Vec<i32> = measurements.get(0..3).unwrap().to_vec();

  measurements.get(3..).unwrap().iter().for_each(|value| {
    let previous_sum = previous.iter().sum::<i32>();
    let sum: i32 = previous.iter().rev().take(2).sum::<i32>() + value;
    if sum > previous_sum {
      increases += 1;
    }
    previous.remove(0);
    previous.push(*value);
  });

  println!("Part Two Status Report: Depth increased {} times!", increases);
}
