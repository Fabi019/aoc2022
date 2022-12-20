static INPUT: &str = include_str!("../../assets/day20.txt");

fn main() {
    let numbers = INPUT
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", solve(&numbers, 1, 1));
    println!("Part 2: {}", solve(&numbers, 811589153, 10));
}

fn solve(numbers: &[i64], key: i64, repeats: i64) -> i64 {
    let len = numbers.len() as i64;
    let numbers = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| (i, n * key))
        .collect::<Vec<_>>();

    let mut new = numbers.clone();

    for _ in 1..=repeats {
        for p @ (i, n) in &numbers {
            let i = new.iter().position(|(_i, _)| _i == i).unwrap();

            let mut j = (i as i64 + n) % (len - 1);
            if j < 0 {
                j = len + j - 1
            }

            new.remove(i);
            new.insert(j as usize, *p);
        }
    }

    let o = new.iter().position(|&(_, n)| n == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|&i| new[(o + i) % new.len()].1)
        .sum()
}
