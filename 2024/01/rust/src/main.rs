use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
  let input = read_to_string("input.txt").expect("Failed to read input file");
  let (first_list, second_list): (Vec<_>, Vec<_>) = input
    .split("\n")
    .flat_map(|row| row.split(" "))
    .filter(|location| !location.is_empty())
    .map(|location| location.parse::<i32>().expect("Failed to convert location to integer"))
    .enumerate()
    .partition(|(index, _)| index % 2 == 0);

  let first_list = to_sorted_list(&first_list);
  let second_list = to_sorted_list(&second_list);

  let total_distance = calculate_total_distance(&first_list, &second_list);
  println!("[Part 1] 🤖 Beep, Boop... calculated a total distance of {total_distance}");

  let similarity_score = calculate_similarity_score(&first_list, &second_list);
  println!("[Part 2] 🤖 Beep, Boop... calculated a similarity score of {similarity_score}");
}

fn calculate_similarity_score(first_list: &Vec<&i32>, second_list: &Vec<&i32>) -> i32 {
  let counts = second_list.into_iter().counts_by(|location| *location);
  first_list
    .into_iter()
    .map(|location| *location * (*counts.get(location).unwrap_or(&0) as i32))
    .sum::<i32>()
}

fn calculate_total_distance(first_list: &Vec<&i32>, second_list: &Vec<&i32>) -> i32 {
  first_list
    .iter()
    .zip(second_list)
    .map(|(a, b)| (*a - *b).abs())
    .sum::<i32>()
}

fn to_sorted_list(input: &Vec<(usize, i32)>) -> Vec<&i32> {
  let mut list: Vec<&i32> = input.into_iter().map(|(_, location)| location).collect();
  list.sort();
  list
}
