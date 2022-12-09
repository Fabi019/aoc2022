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
    visited.insert(tails.last().unwrap().clone());
        
    println!("== Initial State ==");
    //print_grid(head, &tails);

    for (dir, steps) in instructions {
        let (dx, dy) = match dir {
            "D" => (0, -1),
            "U" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Invalid direction"),
        };

        println!("== {} {} ==", dir, steps);
        println!();

        for _ in 0..steps {
            head.0 += dx;
            head.1 += dy;

            let mut prev_tail = head;

            println!("Head: {:?}", head);

            let mut _tails = tails.clone();
            for (idx, tail) in _tails.iter_mut().enumerate() {
                let tx = prev_tail.0 - tail.0;
                let ty = prev_tail.1 - tail.1;

                println!("Tail: {} at {:?}", idx, tail);
                println!("tx: {} ty: {}", tx, ty);

                if tx >= 2 {
                    tail.0 += 1;
                    if ty >= 1 {
                        tail.1 += 1;
                    } else if ty <= -1 {
                        tail.1 -= 1;
                    }
                } else if tx <= -2 {
                    tail.0 -= 1;
                    if ty >= 1 {
                        tail.1 += 1;
                    } else if ty <= -1 {
                        tail.1 -= 1;
                    }
                } else if ty >= 2 {
                    tail.1 += 1;
                    if tx >= 1 {
                        tail.0 += 1;
                    } else if tx <= -1 {
                        tail.0 -= 1;
                    }
                } else if ty <= -2 {
                    tail.1 -= 1;
                    if tx >= 1 {
                        tail.0 += 1;
                    } else if tx <= -1 {
                        tail.0 -= 1;
                    }
                }

                prev_tail = *tail;
            }
            tails = _tails;

            visited.insert(tails.last().unwrap().clone());

            //print_grid(head, &tails);
        }
    }

    println!("Fields visited: {}", visited.len())
}

fn print_grid(head: (i32, i32), tails: &[(i32, i32)]) {
    let mut grid = vec![vec!['.'; 30]; 30];
    grid[6][12] = 's';
    for (idx, tail) in tails.iter().enumerate().rev() {
        grid[6 + tail.1 as usize][ 12 + tail.0 as usize] = (idx + 1).to_string().chars().next().unwrap();
    }
    grid[6 + head.1 as usize][ 12 + head.0 as usize] = 'H';
    grid.reverse();
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
    println!();
}