static INPUT: &str = include_str!("../../assets/day01.txt");

fn main() {
  let mut elfs = Vec::new();
  let mut current_sum = 0;

  INPUT.lines().for_each(|line| { 
    if line.is_empty() {
      elfs.push(current_sum);
      current_sum = 0;
    } else {
      current_sum += line.parse::<i32>().unwrap();
    }
  });

  println!("Maximum Calories: {}", elfs.iter().max().unwrap());

  elfs.sort();
  elfs.reverse();

  println!("Top 3 Total Calories: {}", elfs.iter().take(3).sum::<i32>());
}