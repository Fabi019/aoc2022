static INPUT: &str = include_str!("../../assets/day25.txt");

fn main() {
    let sum = INPUT.lines().fold(0, |sum, line| sum + from_snafu(line));
    let snafu = to_snafu(sum);
    println!("Part 1: {snafu}");
}

fn from_snafu(input: &str) -> i64 {
    input.chars().rev().enumerate().fold(0, |sum, (i, c)| {
        let exp = i64::pow(5, i as u32);
        sum + match c {
            '-' => -exp,
            '=' => -2 * exp,
            _ => c.to_digit(5).unwrap() as i64 * exp,
        }
    })
}

fn to_snafu(dec: i64) -> String {
    let mut n = dec;
    let mut snafu = String::new();
    while n > 0 {
        let (v, c) = match n % 5 {
            0 => (0, '0'),
            1 => (1, '1'),
            2 => (2, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => unreachable!(),
        };
        if v < 0 {
            n += 5;
        }
        n /= 5;
        snafu = c.to_string() + &snafu;
    }
    snafu
}
