use std::fs;

fn main() {
  let content = fs::read_to_string("input.txt").expect("Failed to read input file");
  let crap_submarines: Vec<i32> = content
    .split(",")
    .map(|submarine| submarine.parse::<i32>().expect("Failed to convert to integer"))
    .collect();

  let min = crap_submarines.iter().min().unwrap();
  let max = crap_submarines.iter().max().unwrap();

  println!(
    "Lowest with steady fuel costs: {}",
    (*min..=*max)
      .map(|position| crap_submarines
        .iter()
        .fold(0, |acc, submarine| acc + i32::abs(submarine - position)))
      .min()
      .unwrap()
  );

  println!(
    "Lowest with increasing fuel costs: {}",
    (*min..=*max)
      .map(|position| crap_submarines.iter().fold(0, |acc, submarine| {
        let distance = i32::abs(submarine - position);
        acc + ((distance.pow(2) + distance) / 2)
      }))
      .min()
      .unwrap()
  );
}
