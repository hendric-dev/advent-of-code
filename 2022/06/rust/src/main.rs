fn main() {
  let (packet_marker, message_marker, _) = std::fs::read_to_string("input.txt")
    .expect("Failed to read input file")
    .chars()
    .enumerate()
    .fold((0, 0, vec![]), |mut acc, (index, char)| {
      acc.2.push(char);
      let mut last_four: Vec<&char> = acc.2.iter().rev().take(4).collect();
      let mut last_fourteen: Vec<&char> = acc.2.iter().rev().take(14).collect();
      if acc.0 == 0 && unique(&mut last_four).len() == 4 { acc.0 = index + 1; }
      if acc.1 == 0 && unique(&mut last_fourteen).len() == 14 { acc.1 = index + 1; }
      acc
    });

  println!("The first packet marker appears after position {packet_marker}");
  println!("The first message marker appears after position {message_marker}");
}

fn unique<'a>(vector: &'a mut Vec<&char>) -> &'a Vec<&'a char> {
  vector.sort_unstable();
  vector.dedup();
  vector
}
