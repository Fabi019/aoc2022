static INPUT: &str = include_str!("../../assets/day04.txt");

fn main() {
  let mut fully_contained = 0;
  let mut partially_contained = 0;

  for line in INPUT.lines() {
    let (left, right) = line.split_once(',').unwrap();

    let left_bound = bounds(left);
    let right_bound = bounds(right);

    // Part one: Fully contained
    if left_bound.0 <= right_bound.0 && left_bound.1 >= right_bound.1 {
      fully_contained += 1;
    } else if right_bound.0 <= left_bound.0 && right_bound.1 >= left_bound.1 {
      fully_contained += 1;
    }

    // Part two: Partially contained
    if left_bound.0 <= right_bound.0 && left_bound.1 >= right_bound.0 {
      partially_contained += 1;
    } else if right_bound.0 <= left_bound.0 && right_bound.1 >= left_bound.0 {
      partially_contained += 1;
    }
  }

  println!("Fully contained: {}", fully_contained);
  println!("Partially contained: {}", partially_contained);
}

fn bounds(range: &str) -> (u32, u32) {
  let (left, right) = range.split_once('-').unwrap();
  (left.parse().unwrap(), right.parse().unwrap())
}