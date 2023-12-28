use std::slice::Iter;

#[derive(Debug)]
pub struct Hands(Vec<Hand>);

#[derive(Debug)]
pub struct Hand {
  pub bid: i32,
  pub cards: String,
  pub hand_type: HandType,
  pub rank: i32,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandType {
  FiveOfAKind = 7,
  FourOfAKind = 6,
  FullHouse = 5,
  ThreeOfAKind = 4,
  TwoPair = 3,
  Pair = 2,
  HighCard = 1,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
  Normal,
  Joker,
}

impl Hands {
  pub fn from_input(input: &str, mode: Mode) -> Self {
    Self(input.lines().map(|line| Hand::from_input(line, &mode)).collect())
  }

  pub fn get_winnings(&self) -> i32 {
    self
      .iter()
      .enumerate()
      .map(|(index, hand)| hand.bid * ((index as i32) + 1))
      .sum()
  }

  pub fn iter(&self) -> Iter<Hand> {
    self.0.iter()
  }

  pub fn sort_by_strength(&mut self) {
    self.0.sort_by(|a, b| {
      if a.hand_type == b.hand_type {
        a.rank.cmp(&b.rank)
      } else {
        a.hand_type.cmp(&b.hand_type)
      }
    });
  }
}

impl Hand {
  pub fn from_input(line: &str, mode: &Mode) -> Self {
    let mut split = line.split_whitespace();
    let cards = String::from(split.next().expect("Failed to extract cards from input"));
    let bid: i32 = split
      .next()
      .expect("Failed to extract bid from input")
      .parse()
      .expect("Failed to parse bid into integer");
    let hand_type = Self::determine_hand_type(&cards, mode);
    let rank = Self::determine_rank(&cards, &mode);

    Self {
      bid,
      cards,
      hand_type,
      rank,
    }
  }

  fn determine_hand_type(cards: &str, mode: &Mode) -> HandType {
    let mut hand_type = HandType::HighCard;
    let mut unique_cards: Vec<char> = cards.chars().collect();
    unique_cards.sort();
    unique_cards.dedup();
    if *mode == Mode::Joker {
      unique_cards.retain(|&c| c != 'J');
    }

    unique_cards.iter().for_each(|card| match cards.matches(*card).count() {
      5 => hand_type = HandType::FiveOfAKind,
      4 => hand_type = HandType::FourOfAKind,
      3 => {
        if hand_type == HandType::Pair {
          hand_type = HandType::FullHouse
        } else {
          hand_type = HandType::ThreeOfAKind
        }
      }
      2 => {
        if hand_type == HandType::Pair {
          hand_type = HandType::TwoPair
        } else if hand_type == HandType::ThreeOfAKind {
          hand_type = HandType::FullHouse
        } else {
          hand_type = HandType::Pair
        }
      }
      1 => (),
      _ => panic!("Invalid card: {}", card),
    });

    if *mode == Mode::Joker {
      match cards.matches('J').count() {
        5 => hand_type = HandType::FiveOfAKind,
        4 => hand_type = HandType::FiveOfAKind,
        3 => {
          if hand_type == HandType::Pair {
            hand_type = HandType::FiveOfAKind
          } else {
            hand_type = HandType::FourOfAKind
          }
        }
        2 => {
          if hand_type == HandType::HighCard {
            hand_type = HandType::ThreeOfAKind
          } else if hand_type == HandType::Pair {
            hand_type = HandType::FourOfAKind
          } else if hand_type == HandType::ThreeOfAKind {
            hand_type = HandType::FiveOfAKind
          }
        }
        1 => {
          if hand_type == HandType::HighCard {
            hand_type = HandType::Pair
          } else if hand_type == HandType::Pair {
            hand_type = HandType::ThreeOfAKind
          } else if hand_type == HandType::TwoPair {
            hand_type = HandType::FullHouse
          } else if hand_type == HandType::ThreeOfAKind {
            hand_type = HandType::FourOfAKind
          } else if hand_type == HandType::FourOfAKind {
            hand_type = HandType::FiveOfAKind
          }
        }
        _ => (),
      }
    }

    hand_type
  }

  fn determine_rank(cards: &str, mode: &Mode) -> i32 {
    cards
      .chars()
      .map(|card| match card {
        'A' => "14".to_string(),
        'K' => "13".to_string(),
        'Q' => "12".to_string(),
        'J' => {
          if *mode == Mode::Joker {
            "01".to_string()
          } else {
            "11".to_string()
          }
        }
        'T' => "10".to_string(),
        _ => format!("0{card}"),
      })
      .collect::<Vec<String>>()
      .join("")
      .parse()
      .expect("Failed to convert card rank to integer")
  }
}
