use std::collections::HashSet;

static INPUT: &str = include_str!("../../assets/day06.txt");

fn main() {
  // Part 1: len == 4
  // Part 2: len == 14
  const PATTERN_LEN: usize = 14;

  let chars = INPUT.chars().collect::<Vec<_>>();

  for (idx, pattern) in chars.windows(PATTERN_LEN).enumerate() {
    if distinct(pattern) {
      println!("{}: {:?}", idx + PATTERN_LEN, pattern);
      break;
    }
  }
}

fn distinct(pattern: &[char]) -> bool {
  let mut chars = HashSet::new();
  for c in pattern {
    if !chars.insert(c) {
      return false
    }
  }
  true
}