use std::collections::BinaryHeap;

static INPUT: &str = include_str!("../../assets/day01.txt");

fn main() {
  let mut elfs = BinaryHeap::new();
  let mut current_sum = 0;

  for line in INPUT.lines() { 
    if line.is_empty() {
      elfs.push(current_sum);
      current_sum = 0;
    } else {
      current_sum += line.parse::<i32>().unwrap();
    }
  }

  println!("Maximum Calories: {}", elfs.peek().unwrap());
  println!("Top 3 Total Calories: {}", elfs.iter().take(3).sum::<i32>());
}