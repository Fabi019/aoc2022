static INPUT: &str = include_str!("../../assets/day02.txt");

#[derive(Debug, Clone, Copy)]
enum Type {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
  let mut total_score_1 = 0;
  let mut total_score_2 = 0;

  for line in INPUT.lines() {
    let (a, b) = line.split_once(' ').unwrap();

    let opponent = match a {
      "A" => Type::Rock,
      "B" => Type::Paper,
      "C" => Type::Scissors,
      _ => unreachable!("Invalid input")
    };

    // Part one
    // X = rock, Y = paper, Z = scissors
    let player = match b {
      "X" => Type::Rock,
      "Y" => Type::Paper,
      "Z" => Type::Scissors,
      _ => unreachable!("Invalid input")
    };
    
    total_score_1 += player as i32;
    total_score_1 += round_score(opponent, player);

    // Part two
    // X = lose, Y = draw, Z = win
    let player = match (b, opponent) {
      ("X", Type::Rock) => Type::Scissors,
      ("X", Type::Paper) => Type::Rock,
      ("X", Type::Scissors) => Type::Paper,
      ("Y", Type::Rock) => Type::Rock,
      ("Y", Type::Paper) => Type::Paper,
      ("Y", Type::Scissors) => Type::Scissors,
      ("Z", Type::Rock) => Type::Paper,
      ("Z", Type::Paper) => Type::Scissors,
      ("Z", Type::Scissors) => Type::Rock,
      _ => unreachable!("Invalid input")
    };

    total_score_2 += player as i32;
    total_score_2 += round_score(opponent, player);
  }

  println!("Total score 1: {}", total_score_1);
  println!("Total score 2: {}", total_score_2);
}

fn round_score(opponent: Type, player: Type) -> i32 {
  match (opponent, player) {
    (Type::Rock, Type::Paper) => 6,
    (Type::Rock, Type::Scissors) => 0,
    (Type::Paper, Type::Rock) => 0,
    (Type::Paper, Type::Scissors) => 6,
    (Type::Scissors, Type::Rock) => 6,
    (Type::Scissors, Type::Paper) => 0,
    (Type::Rock, Type::Rock) => 3,
    (Type::Paper, Type::Paper) => 3,
    (Type::Scissors, Type::Scissors) => 3
  }
}