use std::collections::HashSet;

static INPUT: &str = include_str!("../../assets/day09.txt");

fn main() {
    // Part 1: 1
    // Part 2: 9
    const TAIL_LENGTH: usize = 9;

    let instructions = INPUT
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(' ').unwrap();
            (dir, steps.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut head = (0, 0);
    let mut tails = vec![(0, 0); TAIL_LENGTH];

    let mut visited = HashSet::new();
    visited.insert(head);

    for (dir, steps) in instructions {
        let (dx, dy) = match dir {
            "D" => (0, -1),
            "U" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        for _ in 0..steps {
            head.0 += dx;
            head.1 += dy;

            let mut prev_tail = head;

            let mut _tails = tails.clone();
            for tail in _tails.iter_mut() {
                let tx: i32 = prev_tail.0 - tail.0;
                let ty: i32 = prev_tail.1 - tail.1;

                if tx >= 2 || tx <= -2 {
                    tail.0 += tx.signum();
                    tail.1 += ty.signum();
                } else if ty >= 2 || ty <= -2 {
                    tail.1 += ty.signum();
                    tail.0 += tx.signum();
                }

                prev_tail = *tail;
            }
            tails = _tails;

            visited.insert(tails.last().unwrap().clone());
        }
    }

    println!("Fields visited: {}", visited.len())
}