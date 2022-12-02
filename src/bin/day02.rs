static INPUT: &str = include_str!("../../assets/day02.txt");

#[derive(Debug, Clone, Copy)]
enum RPS {
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
      "A" => RPS::Rock,
      "B" => RPS::Paper,
      "C" => RPS::Scissors,
      _ => unreachable!("Invalid input")
    };

    // Part one
    // X = rock, Y = paper, Z = scissors
    let player = match b {
      "X" => RPS::Rock,
      "Y" => RPS::Paper,
      "Z" => RPS::Scissors,
      _ => unreachable!("Invalid input")
    };
    
    total_score_1 += player as i32;
    total_score_1 += round_score(opponent, player);

    // Part two
    // X = lose, Y = draw, Z = win
    let player = match (b, opponent) {
      ("X", RPS::Rock) => RPS::Scissors,
      ("X", RPS::Paper) => RPS::Rock,
      ("X", RPS::Scissors) => RPS::Paper,
      ("Y", RPS::Rock) => RPS::Rock,
      ("Y", RPS::Paper) => RPS::Paper,
      ("Y", RPS::Scissors) => RPS::Scissors,
      ("Z", RPS::Rock) => RPS::Paper,
      ("Z", RPS::Paper) => RPS::Scissors,
      ("Z", RPS::Scissors) => RPS::Rock,
      _ => unreachable!("Invalid input")
    };

    total_score_2 += player as i32;
    total_score_2 += round_score(opponent, player);
  }

  println!("Total score 1: {}", total_score_1);
  println!("Total score 2: {}", total_score_2);
}

fn round_score(opponent: RPS, player: RPS) -> i32 {
  match (opponent, player) {
    (RPS::Rock, RPS::Paper) => 6,
    (RPS::Rock, RPS::Scissors) => 0,
    (RPS::Paper, RPS::Rock) => 0,
    (RPS::Paper, RPS::Scissors) => 6,
    (RPS::Scissors, RPS::Rock) => 6,
    (RPS::Scissors, RPS::Paper) => 0,
    (RPS::Rock, RPS::Rock) => 3,
    (RPS::Paper, RPS::Paper) => 3,
    (RPS::Scissors, RPS::Scissors) => 3
  }
}